#![allow(clippy::pedantic)]

use acebau_database::Table;
use acebau_unit::Price;
use async_graphql::Enum;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Display, EnumString, PartialEq, Eq, Enum)]
#[sqlx(type_name = "expense_payment_state", rename_all = "lowercase")]
#[graphql(rename_items = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ExpensePaymentState {
    Unpaid,
    Paid,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "expense")]
#[changeset(setter(prefix = "with"))]
pub struct Expense {
    pub activity_id: Option<Uuid>,
    pub supplier: String,
    pub accounting_date: NaiveDate,
    pub category: String,
    pub description: String,
    pub total_ht: Price,
    pub total_vat: Price,
    pub total_ttc: Price,
    pub payment_state: ExpensePaymentState,
    pub payment_method: String,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
