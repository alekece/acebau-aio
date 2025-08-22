#![allow(clippy::pedantic)]

mod changeset;
mod database;
pub mod models;
mod record;
mod repository;
#[cfg(test)]
mod tests;
pub mod types;

pub use changeset::Changeset;
pub use database::{Database, DatabaseHandle, Executor, Transaction};
pub use record::Record;
pub use repository::{FetchOptions, Repository};
pub use types::{Duration, Percentage, Quantity, Status};

// pub use query::{UpdateQuery};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
