#![allow(clippy::pedantic)]

mod database;
mod record;
mod repository;
#[cfg(test)]
mod tests;
pub mod types;

#[cfg(feature = "derive")]
pub use acebau_database_derive::Table;

pub use database::{Database, DatabaseHandle, Executor, Transaction};
pub use record::Record;
pub use repository::{FetchOptions, Repository, RepositoryError};
