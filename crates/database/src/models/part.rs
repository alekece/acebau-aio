use acebau_unit::{Length, Mass, Time};
use sqlx::FromRow;

use crate::Table;

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "parts")]
#[changeset(setter(prefix = "with"))]
pub struct Part {
    pub name: String,
    pub size: String,
    pub width: Length,
    pub depth: Length,
    pub height: Length,
    pub print_duration: Time,
    pub filament_required: Mass,
    pub complexity_factor: f32,
}
