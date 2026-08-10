use acebau_unit::PricePerEnergy;
use sqlx::FromRow;

use crate::{Table, types::Percentage};

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "printing_environments")]
#[changeset(setter(prefix = "with"))]
pub struct PrintingEnvironment {
    pub name: String,
    pub operating_factor: Percentage,
    pub electricity_cost: PricePerEnergy,
}
