use async_graphql::{Enum, ObjectType, SimpleObject, TypeName};
use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use sqlx::{FromRow, Type};
use strum::Display;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Type, Display, Enum)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[graphql(rename_items = "lowercase")]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum Status {
    Active,
    Draft,
    Archived,
}

/// `Record` represents a database record with metadata attached.
#[derive(Debug, Clone, FromRow, Deref, DerefMut, SimpleObject)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[graphql(name_type)]
pub struct Record<T: ObjectType + TypeName> {
    pub id: Uuid,
    pub status: Status,
    #[deref]
    #[deref_mut]
    #[sqlx(flatten)]
    #[graphql(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    data: T,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<T: ObjectType + TypeName> Record<T> {
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T: ObjectType + TypeName> TypeName for Record<T> {
    fn type_name() -> std::borrow::Cow<'static, str> {
        format!("{}Record", <T as TypeName>::type_name()).into()
    }
}
