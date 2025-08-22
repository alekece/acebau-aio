use acebau_database::{Record, Status};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use uuid::Uuid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum View {
    Full,
    #[default]
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T> {
    id: Uuid,
    status: Status,
    data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,
}

impl<T> Envelope<T> {
    pub fn new(record: Record<T>) -> Self {
        Self {
            id: record.id(),
            status: record.status(),
            metadata: Some(Metadata {
                created_at: record.created_at(),
                updated_at: record.updated_at(),
            }),
            data: record.into_inner(),
        }
    }

    pub fn with_view(record: Record<T>, view: View) -> Self {
        match view {
            View::Full => Self::new(record),
            View::Compact => Self {
                id: record.id(),
                status: record.status(),
                data: record.into_inner(),
                metadata: None,
            },
        }
    }

    pub fn into_compact(self) -> Self {
        Self {
            id: self.id,
            status: self.status,
            data: self.data,
            metadata: None,
        }
    }
}

impl<T> From<Record<T>> for Envelope<T> {
    fn from(record: Record<T>) -> Self {
        Self::new(record)
    }
}
