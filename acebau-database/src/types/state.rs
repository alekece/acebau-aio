#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[repr(i16)]
pub enum State {
    Active,
    Inactive,
    Archived,
}
