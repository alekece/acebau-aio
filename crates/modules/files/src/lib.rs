#![allow(clippy::pedantic)]

use acebau_database::Table;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "file_metadata")]
#[changeset(setter(prefix = "with"))]
pub struct FileMetadata {
    pub storage_key: String,
    pub original_filename: String,
    pub media_type: String,
    pub size_bytes: i64,
    pub owner_kind: String,
    pub owner_id: Uuid,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
