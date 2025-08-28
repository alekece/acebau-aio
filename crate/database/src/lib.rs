#![allow(clippy::pedantic)]

mod executor;
mod database;
pub mod models;
mod record;
mod repository;
#[cfg(test)]
mod tests;
pub mod types;

#[cfg(feature = "derive")]
pub use acebau_database_derive::Table;

pub use executor::Executor;
pub use database::{Database, DatabaseHandle, Transaction};
pub use record::Record;
pub use repository::{FetchOptions, Repository, RepositoryError};

// pub use query::{UpdateQuery};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
