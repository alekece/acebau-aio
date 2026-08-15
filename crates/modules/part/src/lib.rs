#![allow(clippy::pedantic)]

use acebau_database::Table;
use acebau_machine::MachineModel;
use acebau_unit::{Length, Mass, Time};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "printed_piece")]
#[changeset(setter(prefix = "with"))]
pub struct PrintedPiece {
    pub name: String,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "piece_machine_profile")]
#[changeset(setter(prefix = "with"))]
pub struct PieceMachineProfile {
    #[table(relationship(name = piece, target = PrintedPiece))]
    pub piece_id: Uuid,
    #[table(relationship(name = machine_model, target = MachineModel))]
    pub machine_model_id: Uuid,
    pub nozzle_size: Length,
    pub printing_time: Time,
    pub filament_mass: Mass,
    pub plate_capacity: i32,
    pub quality_note: String,
    pub preferred: bool,
    pub excluded: bool,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
