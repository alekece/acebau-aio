use ulid::Ulid;
use uuid::Uuid;

pub trait Generate<T = Self> {
    fn generate() -> T;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "sql", derive(sqlx::Type), sqlx(transparent))]
pub struct OrderedId(Uuid);

impl Generate for OrderedId {
    fn generate() -> Self {
        Self(Ulid::new().into())
    }
}
