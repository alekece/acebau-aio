#![allow(clippy::pedantic)]

pub mod collection;
pub mod entity;
pub mod error;
pub mod format;
pub mod queue;
pub mod record;
pub mod sql;
pub mod types;

use async_trait::async_trait;

pub use collection::{Collection, SearchQuery, UpdateQuery};
pub use entity::Entity;
pub use error::Error;
pub use queue::{PersistenceMode, PullOptions, PullStrategy, Queue};

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait(?Send)]
pub trait Database:  {
    type Transaction: Transaction;

    #[must_use]
    async fn begin_transaction(&self) -> Result<Self::Transaction>;
}

#[async_trait(?Send)]
pub trait Transaction {
    async fn commit(mut self) -> Result<()>;
    async fn rollback(mut self) -> Result<()>;
}

#[async_trait(?Send)]
pub trait Schema<T: Entity, C> {
    async fn create_schema(&self, connection: &mut C) -> Result<()>;
    async fn drop_schema(&self, connection: &mut C) -> Result<()>;
}
