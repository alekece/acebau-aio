use sqlx::FromRow;

use crate::{
    types::{Duration, Length, Mass},
    Table,
};

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "parts")]
#[changeset(setter(prefix = "with"))]
pub struct Part {
    pub name: String,
    pub size: String,
    pub width: Length,
    pub length: Length,
    pub height: Length,
    pub print_duration: Duration,
    pub filament_required: Mass,
    pub complexity_factor: f32,
}
