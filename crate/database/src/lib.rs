#![allow(clippy::pedantic)]

mod database;
pub mod models;
mod record;
mod repository;
#[cfg(test)]
mod tests;
pub mod types;

pub use database::{Database, DatabaseHandle, Executor, Transaction};
pub use record::Record;
pub use repository::{FetchOptions, Repository};

// pub use query::{UpdateQuery};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
