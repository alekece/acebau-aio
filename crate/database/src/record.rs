use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use sqlx::FromRow;
use uuid::Uuid;

use crate::Status;

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metadata {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// `Record` struct represents a database record of a specific type `T` with metadata attached to it.
#[derive(Debug, Clone, FromRow, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Record<T> {
    id: Uuid,
    status: Status,
    #[sqlx(flatten)]
    #[deref]
    #[deref_mut]
    data: T,
    #[sqlx(flatten)]
    metadata: Metadata,
}

impl<T> Record<T> {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.metadata.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.metadata.updated_at
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}

#[derive(Debug, Clone, FromRow, Deref, DerefMut)]
pub struct RecordLite<T> {
    id: Uuid,
    status: Status,
    #[sqlx(flatten)]
    #[deref]
    #[deref_mut]
    data: T,
}
