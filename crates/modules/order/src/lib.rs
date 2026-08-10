#![allow(clippy::pedantic)]

use acebau_catalogue::Variant;
use acebau_database::Table;
use acebau_unit::Price;
use async_graphql::Enum;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "order_source", rename_all = "snake_case")]
#[graphql(rename_items = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderSource {
    ResellerCatalogue,
    Direct,
    Shopify,
    Etsy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "order_state", rename_all = "snake_case")]
#[graphql(rename_items = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    Pending,
    Accepted,
    Rejected,
    InProduction,
    ReadyToShip,
    AwaitingPayment,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "customer_order")]
#[changeset(setter(prefix = "with"))]
pub struct CustomerOrder {
    pub reference: String,
    pub reseller_id: Option<Uuid>,
    pub customer_name: String,
    pub source: OrderSource,
    pub state: OrderState,
    pub requested_on: NaiveDate,
    pub total_ht: Price,
    pub progress_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "order_line")]
#[changeset(setter(prefix = "with"))]
pub struct OrderLine {
    #[table(relationship(name = order, target = CustomerOrder))]
    pub order_id: Uuid,
    #[table(relationship(name = variant, target = Variant))]
    pub variant_id: Uuid,
    pub quantity: i32,
    pub unit_price_ht: Price,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
