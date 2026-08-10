#![allow(clippy::pedantic)]

use acebau_database::Table;
use acebau_unit::Price;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "product")]
#[changeset(setter(prefix = "with"))]
pub struct Product {
    pub name: String,
    pub collection: String,
    pub category: String,
    pub short_description: String,
    pub customizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "variant")]
#[changeset(setter(prefix = "with"))]
pub struct Variant {
    #[table(relationship(name = product, target = Product))]
    pub product_id: Uuid,
    pub display_name: String,
    pub sku: String,
    pub retail_price: Price,
    pub reseller_price: Price,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
