#![allow(clippy::pedantic)]

extern crate self as acebau_database;

mod database;
mod page;
mod record;
mod repository;
#[cfg(test)]
mod tests;

#[cfg(feature = "derive")]
pub use acebau_database_derive::Table;

pub use database::{Database, DatabaseError, DatabaseHandle, Executor, MigrationOptions, Transaction};
pub use page::Page;
pub use record::{Record, Status};
pub use repository::{FetchOptions, Repository, RepositoryError};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
