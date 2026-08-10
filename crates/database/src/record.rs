use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use sqlx::FromRow;
use uuid::Uuid;

use crate::types::Status;

/// `Record` struct represents a database record of a specific type `T` with metadata attached to it.
#[derive(Debug, Clone, FromRow, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Record<T> {
    id: Uuid,
    status: Status,
    #[sqlx(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[deref]
    #[deref_mut]
    data: T,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl<T> Record<T> {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}
