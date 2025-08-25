use uuid::Uuid;

use crate::{Record, types::Status};

#[derive(Debug, Clone)]
pub struct FetchOptions {
    status: Option<Status>,
}

impl FetchOptions {
    pub fn new() -> Self {
        Self { status: None }
    }

    pub fn status(&self) -> Option<Status> {
        self.status
    }
}

impl Default for FetchOptions {
    fn default() -> Self {
        Self {
            status: Some(Status::Active),
        }
    }
}

// pub enum ConstraintError {
//     CheckViolated,
//     UniqueViolated,
//     ForeignKeyViolated,
// }

// impl<E: DatabaseError> From<E> for ConstraintError {
//     fn from(error: E) -> Self {
//         match error {
//             DatabaseError::CheckConstraintViolation { .. } => ConstraintError::CheckViolated,
//             DatabaseError::UniqueConstraintViolation { .. } => ConstraintError::UniqueViolated,
//             DatabaseError::ForeignKeyConstraintViolation { .. } => ConstraintError::ForeignKeyViolated,
//             _ => panic!("Unexpected error type"),
//         }
//     }
// }

// #[derive(Debug, Snafu)]
// pub enum RepositoryError {
//     #[snafu(display("Failed to delete item with ID '{id}': {source}"))]
//     UniqueConstraintViolation { source: sqlx::Error },
//     #[snafu(display("Failed to insert item: {source}"))]
//     Insertion { source: sqlx::Error },
//     #[snafu(display("Failed to update item with ID '{id}': {source}"))]
//     Update { id: Uuid, source: sqlx::Error },
//     NotFound { id: Uuid },
//     Fetch,
// }

pub trait Repository<T> {
    type Error;
    type Changeset;

    fn insert(&mut self, item: &T, status: Status) -> impl Future<Output = Result<Record<T>, Self::Error>>;
    fn update(
        &mut self,
        id: Uuid,
        changeset: &Self::Changeset,
        status: Option<Status>,
    ) -> impl Future<Output = Result<Record<T>, Self::Error>>;
    fn fetch_by_id(&mut self, id: Uuid) -> impl Future<Output = Result<Record<T>, Self::Error>>;
    fn fetch_all(&mut self, options: FetchOptions) -> impl Future<Output = Result<Vec<Record<T>>, Self::Error>>;
    fn delete(&mut self, id: Uuid) -> impl Future<Output = Result<(), Self::Error>>;
    fn status(&mut self, id: Uuid) -> impl Future<Output = Result<Status, Self::Error>>;
}
