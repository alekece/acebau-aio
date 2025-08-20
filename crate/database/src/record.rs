use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use sqlx::{FromRow, Type};
use strum::Display;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Type, Display)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Status {
    Draft,
    Published,
    Archived,
}

/// `Record` struct represents a database row of a specific type `T` with some metadata.
/// It contains a unique identifier, the data itself, and some timestamps.
#[derive(Debug, Clone, FromRow, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Record<T> {
    id: Uuid,
    status: Status,
    #[sqlx(flatten)]
    #[deref]
    #[deref_mut]
    data: T,
    created_at: DateTime<Utc>,
    #[sqlx(default)]
    updated_at: Option<DateTime<Utc>>,
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

    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }
}
