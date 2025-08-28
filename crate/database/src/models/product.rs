use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    types::{Length, Percentage, PreTaxPrice, Mass},
    Table,
};

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "products")]
#[changeset(setter(prefix = "with"))]
pub struct Product {
    pub name: String,
    pub description: Option<String>,
    pub version: i32,
}

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "product_variants")]
#[changeset(setter(prefix = "with"))]
pub struct ProductVariant {
    pub product_id: Uuid,
    pub sku: String,
    pub display_name: Option<String>,
    pub price_ht: PreTaxPrice,
    pub vat_ratio: Percentage,
    pub resale_coefficient: f32,
    pub height: Option<Length>,
    pub width: Option<Length>,
    pub length: Option<Length>,
    pub weight: Option<Mass>,
}

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "product_variant_parts")]
#[changeset(setter(prefix = "with"))]
pub struct ProductVariantPart {
    pub product_variant_id: Uuid,
    pub part_id: Uuid,
    pub filament_id: Uuid,
    pub quantity: i64,
}
