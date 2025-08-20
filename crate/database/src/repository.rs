use uuid::Uuid;

use crate::{Record, Status};

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
            status: Some(Status::Published),
        }
    }
}

pub trait Repository<T> {
    type Error;

    fn insert(&mut self, item: T) -> impl Future<Output = Result<Record<T>, Self::Error>>;
    fn update(&mut self, id: Uuid, item: T) -> impl Future<Output = Result<Record<T>, Self::Error>>;
    fn fetch_by_id(&mut self, id: Uuid) -> impl Future<Output = Result<Record<T>, Self::Error>>;
    fn fetch_all(&mut self, options: FetchOptions) -> impl Future<Output = Result<Vec<Record<T>>, Self::Error>>;
    fn delete(&mut self, id: Uuid) -> impl Future<Output = Result<(), Self::Error>>;
}
