#![allow(clippy::pedantic)]

use acebau_catalogue::Variant;
use acebau_database::{Database, Executor as _, Record, Table};
use acebau_inventory::Supply;
use acebau_machine::Machine;
use acebau_part::PrintedPiece;
use async_graphql::{Context, Enum, InputObject, Object};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "production_state", rename_all = "snake_case")]
#[graphql(rename_items = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProductionState {
    ToPlan,
    ToStart,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "planning_preference", rename_all = "lowercase")]
#[graphql(rename_items = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PlanningPreference {
    Quality,
    Cost,
    Speed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "production_job_state", rename_all = "snake_case")]
#[graphql(rename_items = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProductionJobState {
    ToPrint,
    Printing,
    Done,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "production_task")]
#[changeset(setter(prefix = "with"))]
pub struct ProductionTask {
    pub reference: String,
    #[table(relationship(name = source_variant, target = Variant))]
    pub source_variant_id: Option<Uuid>,
    pub product_quantity: Option<i32>,
    pub linked_order_reference: Option<String>,
    pub deadline: NaiveDate,
    pub state: ProductionState,
    pub planning_preference: PlanningPreference,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "production_task_line")]
#[changeset(setter(prefix = "with"))]
pub struct ProductionTaskLine {
    #[table(relationship(name = production_task, target = ProductionTask))]
    pub production_task_id: Uuid,
    #[table(relationship(name = piece, target = PrintedPiece))]
    pub piece_id: Uuid,
    #[table(relationship(name = filament_supply, target = Supply))]
    pub filament_supply_id: Uuid,
    pub quantity: i32,
    #[table(relationship(name = machine, target = Machine))]
    pub machine_id: Option<Uuid>,
    pub state: ProductionJobState,
    pub started_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    pub failed_quantity: Option<i32>,
    pub actual_waste_grams: Option<f64>,
}

#[derive(Debug, InputObject)]
pub struct ProductionSelectionInput {
    pub piece_id: Uuid,
    pub filament_supply_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, InputObject)]
pub struct CreateProductionInput {
    pub source_variant_id: Option<Uuid>,
    pub product_quantity: Option<i32>,
    pub linked_order_reference: Option<String>,
    pub deadline: NaiveDate,
    pub planning_preference: PlanningPreference,
    pub lines: Vec<ProductionSelectionInput>,
}

#[derive(Debug, InputObject)]
pub struct UpdateProductionBundleInput {
    pub linked_order_reference: Option<String>,
    pub deadline: NaiveDate,
    pub planning_preference: PlanningPreference,
}

#[derive(Debug, InputObject)]
pub struct UpdateProductionJobInput {
    pub piece_id: Uuid,
    pub filament_supply_id: Uuid,
    pub machine_id: Option<Uuid>,
    pub quantity: i32,
}

#[derive(Debug, InputObject)]
pub struct FailProductionJobInput {
    pub reason: String,
    pub failed_quantity: i32,
    pub actual_waste_grams: f64,
}

#[derive(Default)]
pub struct ProductionMutation;

#[Object]
impl ProductionMutation {
    async fn create_production(
        &self,
        context: &Context<'_>,
        input: CreateProductionInput,
    ) -> async_graphql::Result<Record<ProductionTask>> {
        if input.lines.is_empty() {
            return Err("a production must contain at least one variant or piece".into());
        }
        for line in &input.lines {
            if line.quantity <= 0 {
                return Err("production quantities must be greater than zero".into());
            }
        }
        if input.source_variant_id.is_some() != input.product_quantity.is_some()
            || input.product_quantity.is_some_and(|quantity| quantity <= 0)
        {
            return Err("a source variant requires a positive product quantity".into());
        }

        let database = context.data::<Database>()?.clone();
        let mut transaction = database.transaction().await.map_err(graphql_error)?;
        let task = sqlx::query_as::<_, Record<ProductionTask>>(
            "insert into production_task (source_variant_id, product_quantity, linked_order_reference, deadline, state, planning_preference) \
             values ($1, $2, $3, $4, 'to_plan', $5) returning *",
        )
        .bind(input.source_variant_id)
        .bind(input.product_quantity)
        .bind(&input.linked_order_reference)
        .bind(input.deadline)
        .bind(input.planning_preference)
        .fetch_one(transaction.executor())
        .await
        .map_err(graphql_error)?;

        for line in input.lines {
            insert_line(
                &mut transaction,
                task.id,
                line.piece_id,
                line.filament_supply_id,
                line.quantity,
            )
            .await?;
        }

        transaction.commit().await.map_err(graphql_error)?;
        Ok(task)
    }

    async fn update_production_bundle(
        &self,
        context: &Context<'_>,
        id: Uuid,
        input: UpdateProductionBundleInput,
    ) -> async_graphql::Result<Record<ProductionTask>> {
        let mut database = context.data::<Database>()?.clone();
        let task = sqlx::query_as::<_, Record<ProductionTask>>(
            "update production_task set linked_order_reference = $2, deadline = $3, \
             planning_preference = $4, updated_at = now() where id = $1 and state <> 'completed' returning *",
        )
        .bind(id)
        .bind(input.linked_order_reference)
        .bind(input.deadline)
        .bind(input.planning_preference)
        .fetch_optional(database.executor())
        .await
        .map_err(graphql_error)?;

        task.ok_or_else(|| "completed or unknown productions cannot be edited".into())
    }

    async fn delete_production_bundle(&self, context: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let mut database = context.data::<Database>()?.clone();
        let result = sqlx::query("delete from production_task where id = $1 and state <> 'completed'")
            .bind(id)
            .execute(database.executor())
            .await
            .map_err(graphql_error)?;

        if result.rows_affected() == 0 {
            return Err("completed or unknown productions cannot be deleted".into());
        }
        Ok(true)
    }

    async fn update_production_job(
        &self,
        context: &Context<'_>,
        id: Uuid,
        input: UpdateProductionJobInput,
    ) -> async_graphql::Result<Record<ProductionTaskLine>> {
        if input.quantity <= 0 {
            return Err("production quantities must be greater than zero".into());
        }

        let mut database = context.data::<Database>()?.clone();
        let line = sqlx::query_as::<_, Record<ProductionTaskLine>>(
            "update production_task_line as line set piece_id = $2, filament_supply_id = $3, \
             machine_id = $4, quantity = $5, updated_at = now() from production_task as task \
             where line.id = $1 and task.id = line.production_task_id and task.state <> 'completed' \
             and line.state not in ('done', 'printing') returning line.*",
        )
        .bind(id)
        .bind(input.piece_id)
        .bind(input.filament_supply_id)
        .bind(input.machine_id)
        .bind(input.quantity)
        .fetch_optional(database.executor())
        .await
        .map_err(graphql_error)?;

        let line = line.ok_or_else(|| graphql_error("only pending or failed jobs can be edited"))?;
        synchronize_task_state(&mut database, line.production_task_id).await?;
        Ok(line)
    }

    async fn add_production_job(
        &self,
        context: &Context<'_>,
        production_task_id: Uuid,
        input: UpdateProductionJobInput,
    ) -> async_graphql::Result<Record<ProductionTaskLine>> {
        if input.quantity <= 0 {
            return Err("production quantities must be greater than zero".into());
        }

        let mut database = context.data::<Database>()?.clone();
        let line = sqlx::query_as::<_, Record<ProductionTaskLine>>(
            "insert into production_task_line \
             (production_task_id, piece_id, filament_supply_id, machine_id, quantity) \
             select task.id, $2, $3, $4, $5 from production_task as task \
             where task.id = $1 and task.state not in ('completed', 'cancelled') returning *",
        )
        .bind(production_task_id)
        .bind(input.piece_id)
        .bind(input.filament_supply_id)
        .bind(input.machine_id)
        .bind(input.quantity)
        .fetch_optional(database.executor())
        .await
        .map_err(graphql_error)?
        .ok_or_else(|| graphql_error("completed, cancelled or unknown productions cannot receive new jobs"))?;

        synchronize_task_state(&mut database, line.production_task_id).await?;
        Ok(line)
    }

    async fn delete_production_job(&self, context: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let mut database = context.data::<Database>()?.clone();
        let task_id = sqlx::query_scalar::<_, Uuid>(
            "delete from production_task_line as line using production_task as task \
             where line.id = $1 and task.id = line.production_task_id and task.state <> 'completed' \
             and line.state not in ('done', 'printing') returning line.production_task_id",
        )
        .bind(id)
        .fetch_optional(database.executor())
        .await
        .map_err(graphql_error)?
        .ok_or_else(|| graphql_error("only pending or failed jobs can be deleted"))?;

        synchronize_task_state(&mut database, task_id).await?;
        Ok(true)
    }

    async fn start_production_job(
        &self,
        context: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Record<ProductionTaskLine>> {
        let mut database = context.data::<Database>()?.clone();
        let line = sqlx::query_as::<_, Record<ProductionTaskLine>>(
            "update production_task_line as line set state = 'printing', started_at = now(), \
             failure_reason = null, failed_quantity = null, actual_waste_grams = null, updated_at = now() \
             from production_task as task where line.id = $1 and task.id = line.production_task_id \
             and task.state not in ('completed', 'cancelled') and line.state in ('to_print', 'failed') \
             and line.machine_id is not null returning line.*",
        )
        .bind(id)
        .fetch_optional(database.executor())
        .await
        .map_err(graphql_error)?
        .ok_or_else(|| graphql_error("assign a machine before starting a pending job"))?;

        synchronize_task_state(&mut database, line.production_task_id).await?;
        Ok(line)
    }

    async fn complete_production_job(
        &self,
        context: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Record<ProductionTaskLine>> {
        finish_production_job(context, id, None).await
    }

    async fn fail_production_job(
        &self,
        context: &Context<'_>,
        id: Uuid,
        input: FailProductionJobInput,
    ) -> async_graphql::Result<Record<ProductionTaskLine>> {
        if input.reason.trim().is_empty() || input.failed_quantity <= 0 || input.actual_waste_grams < 0.0 {
            return Err("a failure requires a reason, a positive failed quantity and non-negative waste".into());
        }
        finish_production_job(context, id, Some(input)).await
    }
}

async fn insert_line(
    transaction: &mut acebau_database::Transaction<'_>,
    production_task_id: Uuid,
    piece_id: Uuid,
    filament_supply_id: Uuid,
    quantity: i32,
) -> async_graphql::Result<()> {
    sqlx::query(
        "insert into production_task_line \
         (production_task_id, piece_id, filament_supply_id, quantity) values ($1, $2, $3, $4)",
    )
    .bind(production_task_id)
    .bind(piece_id)
    .bind(filament_supply_id)
    .bind(quantity)
    .execute(transaction.executor())
    .await
    .map_err(graphql_error)?;
    Ok(())
}

async fn finish_production_job(
    context: &Context<'_>,
    id: Uuid,
    failure: Option<FailProductionJobInput>,
) -> async_graphql::Result<Record<ProductionTaskLine>> {
    let mut database = context.data::<Database>()?.clone();
    let state = if failure.is_some() { "failed" } else { "done" };
    let line = sqlx::query_as::<_, Record<ProductionTaskLine>>(
        "update production_task_line set state = $2::production_job_state, failure_reason = $3, failed_quantity = $4, \
         actual_waste_grams = $5, updated_at = now() where id = $1 and state = 'printing' returning *",
    )
    .bind(id)
    .bind(state)
    .bind(failure.as_ref().map(|input| input.reason.trim()))
    .bind(failure.as_ref().map(|input| input.failed_quantity))
    .bind(failure.as_ref().map(|input| input.actual_waste_grams))
    .fetch_optional(database.executor())
    .await
    .map_err(graphql_error)?
    .ok_or_else(|| graphql_error("only a printing job can be completed or failed"))?;

    synchronize_task_state(&mut database, line.production_task_id).await?;
    Ok(line)
}

async fn synchronize_task_state(database: &mut Database, task_id: Uuid) -> async_graphql::Result<()> {
    sqlx::query(
        "update production_task set state = case \
            when state = 'cancelled' then state \
            when not exists (select 1 from production_task_line where production_task_id = $1) then 'to_plan' \
            when exists (select 1 from production_task_line where production_task_id = $1 and state = 'printing') then 'in_progress' \
            when not exists (select 1 from production_task_line where production_task_id = $1 and state <> 'done') then 'completed' \
            when exists (select 1 from production_task_line where production_task_id = $1 and state = 'failed') then 'failed' \
            when not exists (select 1 from production_task_line where production_task_id = $1 and machine_id is null) then 'to_start' \
            else 'to_plan' end, updated_at = now() where id = $1",
    )
    .bind(task_id)
    .execute(database.executor())
    .await
    .map_err(graphql_error)?;
    Ok(())
}

fn graphql_error(error: impl std::fmt::Display) -> async_graphql::Error {
    async_graphql::Error::new(error.to_string())
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
