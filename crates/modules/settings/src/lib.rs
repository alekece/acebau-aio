#![allow(clippy::pedantic)]

use acebau_database::Table;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "application_setting")]
#[changeset(setter(prefix = "with"))]
pub struct ApplicationSetting {
    pub default_time_unit: String,
    pub default_mass_unit: String,
    pub default_length_unit: String,
    pub default_power_unit: String,
    pub default_page_size: i32,
    pub electricity_rate: String,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
