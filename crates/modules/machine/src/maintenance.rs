use acebau_database::{Executor, Page, Record, Table};
use acebau_unit::metric::TimeUnit;
use acebau_unit::{Decimal, Time};
use async_graphql::{Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

use crate::{Machine, MachineModel};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "machine_maintenance_kind", rename_all = "snake_case")]
#[graphql(rename_items = "camelCase")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum MachineMaintenanceKind {
    Nozzle,
    AxisCleaning,
    AxisLubrication,
    GeneralCleaning,
    CarbonFilter,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "machine_maintenance_setting")]
#[changeset(setter(prefix = "with"))]
pub struct MachineMaintenanceSetting {
    pub kind: MachineMaintenanceKind,
    pub due_after: Time,
    pub critical_after: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "machine_maintenance", plural = "machine_maintenance_history")]
#[changeset(setter(prefix = "with"))]
pub struct MachineMaintenance {
    #[table(relationship(name = machine, target = Machine))]
    pub machine_id: Uuid,
    pub kind: MachineMaintenanceKind,
    pub performed_at: DateTime<Utc>,
    pub printing_time: Time,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
#[graphql(rename_items = "lowercase")]
pub enum MachineMaintenanceCriticity {
    Normal,
    Due,
    Critical,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct MachineMaintenanceStatus {
    pub kind: MachineMaintenanceKind,
    pub criticity: MachineMaintenanceCriticity,
    pub due_after: Time,
    pub critical_after: Time,
    pub printing_time_since_maintenance: Time,
    pub last_performed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct MachineMaintenanceOverview {
    pub machine_id: Uuid,
    pub statuses: Vec<MachineMaintenanceStatus>,
}

#[derive(FromRow)]
struct LatestMaintenance {
    kind: MachineMaintenanceKind,
    performed_at: DateTime<Utc>,
    printing_time: Time,
}

#[derive(Default)]
pub struct MachineMaintenanceStatusQuery;

#[async_graphql::Object]
impl MachineMaintenanceStatusQuery {
    async fn machine_maintenance_statuses(
        &self,
        ctx: &Context<'_>,
        machine_id: String,
    ) -> async_graphql::Result<Vec<MachineMaintenanceStatus>> {
        let machine_id = Uuid::parse_str(&machine_id)?;
        let mut database = ctx.data::<acebau_database::Database>()?.clone();
        statuses_for(&mut database, machine_id).await
    }

    async fn machine_maintenance_overview(
        &self,
        ctx: &Context<'_>,
        machine_ids: Vec<String>,
    ) -> async_graphql::Result<Vec<MachineMaintenanceOverview>> {
        let mut database = ctx.data::<acebau_database::Database>()?.clone();
        let mut overview = Vec::with_capacity(machine_ids.len());
        for machine_id in machine_ids {
            let machine_id = Uuid::parse_str(&machine_id)?;
            overview.push(MachineMaintenanceOverview {
                machine_id,
                statuses: statuses_for(&mut database, machine_id).await?,
            });
        }
        Ok(overview)
    }

    async fn machine_maintenance_history_for(
        &self,
        ctx: &Context<'_>,
        machine_id: String,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> async_graphql::Result<Page<MachineMaintenance>> {
        let mut database = ctx.data::<acebau_database::Database>()?.clone();
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(10).max(1);
        let machine_id = Uuid::parse_str(&machine_id)?;
        let offset = i64::from((page - 1) * page_size);
        let items = sqlx::query_as(
            "select * from machine_maintenance where machine_id = $1 and status = 'active' order by performed_at desc, created_at desc limit $2 offset $3",
        )
        .bind(machine_id)
        .bind(i64::from(page_size))
        .bind(offset)
        .fetch_all(database.executor())
        .await?;
        let total_items: i64 =
            sqlx::query_scalar("select count(*) from machine_maintenance where machine_id = $1 and status = 'active'")
                .bind(machine_id)
                .fetch_one(database.executor())
                .await?;

        Ok(Page {
            items,
            page,
            page_size,
            total_items: i32::try_from(total_items)?,
            total_pages: if total_items == 0 {
                0
            } else {
                i32::try_from((total_items + i64::from(page_size) - 1) / i64::from(page_size))?
            },
        })
    }
}

async fn statuses_for(
    database: &mut acebau_database::Database,
    machine_id: Uuid,
) -> async_graphql::Result<Vec<MachineMaintenanceStatus>> {
    let machine: Record<Machine> = sqlx::query_as("select * from machine where id = $1")
        .bind(machine_id)
        .fetch_one(database.executor())
        .await?;
    let model: Record<MachineModel> = sqlx::query_as("select * from machine_model where id = $1")
        .bind(machine.model_id)
        .fetch_one(database.executor())
        .await?;
    let settings = sqlx::query_as::<_, MachineMaintenanceSetting>(
            "select kind, due_after, critical_after from machine_maintenance_setting where status = 'active' order by created_at",
        )
        .fetch_all(database.executor())
        .await?;
    let history = sqlx::query_as::<_, LatestMaintenance>(
            "select distinct on (kind) kind, performed_at, printing_time from machine_maintenance where machine_id = $1 and status = 'active' order by kind, performed_at desc, created_at desc",
        )
        .bind(machine_id)
        .fetch_all(database.executor())
        .await?;

    Ok(settings
        .into_iter()
        .filter(|setting| setting.kind != MachineMaintenanceKind::CarbonFilter || model.has_carbon_filter)
        .map(|setting| {
            let latest = history.iter().find(|entry| entry.kind == setting.kind);
            let baseline = latest.map_or(Decimal::ZERO, |entry| {
                entry.printing_time.convert_to(TimeUnit::Minute).value()
            });
            let elapsed_value =
                (machine.printing_time.convert_to(TimeUnit::Minute).value() - baseline).max(Decimal::ZERO);
            let elapsed = Time::with_unit(elapsed_value, TimeUnit::Minute);
            let criticity = maintenance_criticity(elapsed, &setting);

            MachineMaintenanceStatus {
                kind: setting.kind,
                criticity,
                due_after: setting.due_after,
                critical_after: setting.critical_after,
                printing_time_since_maintenance: elapsed,
                last_performed_at: latest.map(|entry| entry.performed_at),
            }
        })
        .collect())
}

fn maintenance_criticity(elapsed: Time, setting: &MachineMaintenanceSetting) -> MachineMaintenanceCriticity {
    let elapsed = elapsed.convert_to(TimeUnit::Minute).value();
    if elapsed > setting.critical_after.convert_to(TimeUnit::Minute).value() {
        MachineMaintenanceCriticity::Critical
    } else if elapsed >= setting.due_after.convert_to(TimeUnit::Minute).value() {
        MachineMaintenanceCriticity::Due
    } else {
        MachineMaintenanceCriticity::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setting() -> MachineMaintenanceSetting {
        MachineMaintenanceSetting {
            kind: MachineMaintenanceKind::Nozzle,
            due_after: Time::with_unit(500, TimeUnit::Hour),
            critical_after: Time::with_unit(1_000, TimeUnit::Hour),
        }
    }

    #[test]
    fn maintenance_becomes_due_at_start_and_critical_after_limit() {
        let setting = setting();

        assert_eq!(
            maintenance_criticity(Time::with_unit(499, TimeUnit::Hour), &setting),
            MachineMaintenanceCriticity::Normal
        );
        assert_eq!(
            maintenance_criticity(Time::with_unit(500, TimeUnit::Hour), &setting),
            MachineMaintenanceCriticity::Due
        );
        assert_eq!(
            maintenance_criticity(Time::with_unit(1_000, TimeUnit::Hour), &setting),
            MachineMaintenanceCriticity::Due
        );
        assert_eq!(
            maintenance_criticity(Time::with_unit(1_001, TimeUnit::Hour), &setting),
            MachineMaintenanceCriticity::Critical
        );
    }
}
