#![allow(clippy::pedantic)]

use acebau_catalogue::Variant;
use acebau_database::Table;
use acebau_inventory::Supply;
use acebau_part::PrintedPiece;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// One concrete printed-piece and filament requirement in a variant recipe.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "recipe_item")]
#[changeset(setter(prefix = "with"))]
pub struct RecipeItem {
    #[table(relationship(name = variant, target = Variant))]
    pub variant_id: Uuid,
    #[table(relationship(name = piece, target = PrintedPiece))]
    pub piece_id: Uuid,
    #[table(relationship(name = filament_supply, target = Supply))]
    pub filament_supply_id: Uuid,
    pub quantity: i32,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
