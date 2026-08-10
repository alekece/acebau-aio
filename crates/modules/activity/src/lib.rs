#![allow(clippy::pedantic)]

use acebau_database::Table;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "activity", plural = "activities")]
#[changeset(setter(prefix = "with"))]
pub struct Activity {
    pub name: String,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
