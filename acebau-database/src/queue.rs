use std::num::NonZeroUsize;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use strum::EnumString;

use crate::{record::Record, Entity, Result, Schema};

pub struct Metadata {
    pub created_at: DateTime<Utc>,
}

/// Indicates whether the item should be removed from the queue when being pulled.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceMode {
    Keep,
    #[default]
    Remove,
}

/// The strategy to use when pulling an item from the queue.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum PullStrategy {
    #[default]
    #[strum(serialize = "fifo")]
    FirstInFirstOut,
    #[strum(serialize = "lifo")]
    LastInFirstOut,
    Random,
}

/// Options for pulling an item from the queue.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PullOptions {
    pub persistence_mode: PersistenceMode,
    pub pull_strategy: PullStrategy,
}

impl PullOptions {
    /// Set the queue to use the first-in-first-out pull strategy.
    pub fn fifo(mut self) -> Self {
        self.pull_strategy = PullStrategy::FirstInFirstOut;

        self
    }

    /// Set the queue to use the last-in-first-out pull strategy.
    pub fn lifo(mut self) -> Self {
        self.pull_strategy = PullStrategy::LastInFirstOut;

        self
    }

    /// Set the queue to use the random pull strategy.
    pub fn random(mut self) -> Self {
        self.pull_strategy = PullStrategy::Random;

        self
    }

    /// Set the queue to keep the item when pulling it.
    pub fn keep(mut self) -> Self {
        self.persistence_mode = PersistenceMode::Keep;

        self
    }

    /// Set the queue to remove the item when pulling it.
    pub fn remove(mut self) -> Self {
        self.persistence_mode = PersistenceMode::Remove;

        self
    }
}

#[async_trait(?Send)]
pub trait Queue<T: Entity, C>: Schema<T, C> {
    async fn push(&self, connection: &mut C, entity: &T) -> Result<T::Id>;
    async fn pull(&self, connection: &mut C, options: PullOptions) -> Result<Option<Record<T, Metadata>>>;
    async fn pull_many(
        &self,
        connection: &mut C,
        options: PullOptions,
        limit: NonZeroUsize,
    ) -> Result<Vec<Record<T, Metadata>>>;
    async fn delete(&self, connection: &mut C, id: T::Id) -> Result<()>;
    async fn clear_pulled(&self, connection: &mut C) -> Result<()>;
    async fn len(&self, connection: &mut C) -> Result<usize>;
}
