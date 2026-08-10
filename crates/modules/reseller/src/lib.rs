#![allow(clippy::pedantic)]

use acebau_database::Table;
use async_graphql::Enum;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "reseller_relationship", rename_all = "lowercase")]
#[graphql(rename_items = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ResellerRelationship {
    Prospect,
    Approved,
    Paused,
    Closed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "reseller")]
#[changeset(setter(prefix = "with"))]
pub struct Reseller {
    pub business_name: String,
    pub city: String,
    pub country: String,
    pub relationship: ResellerRelationship,
    pub primary_email: String,
    pub next_action_date: Option<NaiveDate>,
    pub next_action: String,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
