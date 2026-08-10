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
#[sqlx(type_name = "invoice_payment_state", rename_all = "lowercase")]
#[graphql(rename_items = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum InvoicePaymentState {
    Imported,
    Unpaid,
    Paid,
    Credited,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Table)]
#[table(name = "imported_invoice")]
#[changeset(setter(prefix = "with"))]
pub struct ImportedInvoice {
    pub external_platform: String,
    pub external_identifier: String,
    pub number: String,
    pub customer: String,
    pub activity_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub issued_on: NaiveDate,
    pub due_on: NaiveDate,
    pub total_ht: Price,
    pub total_vat: Price,
    pub total_ttc: Price,
    pub payment_state: InvoicePaymentState,
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
