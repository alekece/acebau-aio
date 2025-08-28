use sqlx::FromRow;

use crate::{types::Percentage, Table};

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "printing_environments")]
#[changeset(setter(prefix = "with"))]
pub struct PrintingEnvironment {
    pub name: String,
    pub operating_factor: Percentage,
    pub electricity_cost_per_kwh: f32,
}
