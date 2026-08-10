#![allow(clippy::pedantic)]

use acebau_database::Table;
use acebau_unit::{Mass, Price};
use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "supply_kind", rename_all = "snake_case")]
#[graphql(rename_items = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SupplyKind {
    Filament,
    ProductionMaterial,
    ProductPackaging,
    ShippingPackaging,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "spool_state", rename_all = "lowercase")]
#[graphql(rename_items = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SpoolState {
    Sealed,
    Open,
    Empty,
    Discarded,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "supply", plural = "supplies")]
#[changeset(setter(prefix = "with"))]
pub struct Supply {
    pub name: String,
    pub reference: String,
    pub kind: SupplyKind,
    pub base_unit: String,
    pub available_quantity: f64,
    pub low_stock_threshold: f64,
    pub target_quantity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "filament_spool")]
#[changeset(setter(prefix = "with"))]
pub struct FilamentSpool {
    #[table(relationship(name = supply, target = Supply))]
    pub supply_id: Uuid,
    pub internal_reference: String,
    pub initial_weight: Mass,
    pub remaining_weight: Mass,
    pub received_cost: Price,
    pub state: SpoolState,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
