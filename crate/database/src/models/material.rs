use acebau_unit::Price;
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::Table;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Type)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Unit {
    Gram,
    Meter,
    Piece,
}

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "materials")]
#[changeset(setter(prefix = "with"))]
pub struct Material {
    pub name: String,
    #[table(rename = "type")]
    pub material_type: String,
    pub unit: Unit,
}

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "material_providers")]
#[changeset(setter(prefix = "with"))]
pub struct MaterialProvider {
    pub material_id: Uuid,
    pub provider_name: String,
    pub url: String,
    pub bundle_size: i64,
    pub unit_price: Price,
    pub bulk_price: Option<Price>,
}
