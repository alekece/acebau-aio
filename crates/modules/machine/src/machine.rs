use acebau_database::{Repository, Table};
use async_graphql::{Context, Enum, Error as GraphQLError};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

use acebau_unit::metric::TimeUnit;
use acebau_unit::{Decimal, Length, Power, Price, PricePerTime, Time};

const DEFAULT_MACHINE_USAGE: Decimal = Decimal::from_parts(5, 0, 0, false, 1);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "machine_state", rename_all = "lowercase")]
#[graphql(rename_items = "lowercase")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "lowercase")]
pub enum MachineState {
    #[graphql(name = "available")]
    Available,
    #[graphql(name = "running")]
    Running,
    #[graphql(name = "maintenance")]
    Maintenance,
    #[graphql(name = "broken")]
    Broken,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "machine_model")]
#[changeset(setter(prefix = "with"))]
pub struct MachineModel {
    pub brand: String,
    pub name: String,
    pub maintenance_cost: PricePerTime,
    pub lifetime: Time,
    pub average_power: Power,
    pub has_carbon_filter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(
    name = "machine",
    resolver(name = usage_cost, method = resolve_usage_cost, ty = "PricePerTime")
)]
#[changeset(setter(prefix = "with"))]
pub struct Machine {
    #[table(relationship(name = model, target = MachineModel))]
    pub model_id: Uuid,
    pub surname: String,
    pub purchase_cost: Price,
    pub nozzle_size: Length,
    pub printing_time: Time,
    pub state: MachineState,
}

impl Machine {
    async fn resolve_usage_cost(&self, ctx: &Context<'_>) -> Result<PricePerTime, GraphQLError> {
        let mut database = ctx.data::<acebau_database::Database>()?.clone();
        let model = database.repository::<MachineModel>().fetch_by_id(self.model_id).await?;
        let lifetime = model.lifetime.convert_to(TimeUnit::Hour);
        let expected_printing_time = lifetime * DEFAULT_MACHINE_USAGE;
        let maintenance_cost = model.maintenance_cost * model.lifetime;
        let total_cost = self.purchase_cost + maintenance_cost;

        Ok(total_cost / expected_printing_time)
    }
}
