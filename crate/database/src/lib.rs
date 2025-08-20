#![allow(clippy::pedantic)]

mod database;
mod executor;
pub mod models;
mod record;
#[cfg(test)]
mod test;
pub mod types;
mod repository;

pub use database::{Database, Transaction};
pub use executor::Executor;
pub use models::product::{Product, ProductRepository, ProductError};
pub use record::{Record, Status};
pub use repository::Repository;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
