use acebau_database::Table;
use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

use acebau_unit::{Power, Price, PricePerTime, Time};

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
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "machine")]
#[changeset(setter(prefix = "with"))]
pub struct Machine {
    #[table(relationship(name = model, target = MachineModel))]
    pub model_id: Uuid,
    pub surname: String,
    #[table(skip)]
    pub purchase_cost: Price,
    pub printing_time: Time,
    pub state: MachineState,
}
