use std::{marker::PhantomData, num::NonZeroUsize};

use async_trait::async_trait;

use crate::{record::Record, types::State, Entity, Result, Schema};

pub struct Metadata {
    pub state: State,
    pub keywords: Vec<String>,
}

#[async_trait(?Send)]
pub trait Collection<T: Entity, C>: Schema<T, C> {
    async fn insert(&self, connection: &mut C, entity: &T) -> Result<T::Id>;
    async fn update<'a>(&self, connection: &mut C, id: &T::Id, query: UpdateQuery<'a, T>) -> Result<()>;
    async fn delete(&self, connection: &mut C, id: &T::Id) -> Result<()>;
    async fn delete_all(&self, connection: &mut C) -> Result<()>;
    async fn fetch_one(&self, connection: &mut C, id: &T::Id) -> Result<Record<T, Metadata>>;
    async fn fetch_many(&self, connection: &mut C, ids: &[T::Id]) -> Result<Vec<Record<T, Metadata>>>;
    async fn fetch_all(&self, connection: &mut C) -> Result<Vec<Record<T, Metadata>>>;
    // async fn find<'a>(&self, connection: &mut C, query: SearchQuery<'a, T>) -> Result<Vec<Record<T>>>;
}

pub enum Search<'a, T> {
    EqualTo(T),
    All(&'a [T]),
    Any(&'a [T]),
}

#[derive(Default)]
pub struct SearchQuery<'a, T: Entity> {
    pub(crate) state: Option<Search<'a, State>>,
    pub(crate) keywords: Option<Search<'a, String>>,
    pub(crate) limit: Option<NonZeroUsize>,
    _marker: PhantomData<T>,
}

impl<T: Entity> SearchQuery<'_, T> {
    pub fn is_empty(&self) -> bool {
        self.state.is_none() && self.keywords.is_none() && self.limit.is_none()
    }
}

impl<'a, T: Entity> SearchQuery<'a, T> {
    pub fn state(&mut self, search: Search<'a, State>) -> &mut Self {
        self.state = Some(search);

        self
    }

    pub fn keywords(&mut self, search: Search<'a, String>) -> &mut Self {
        self.keywords = Some(search);

        self
    }

    pub fn limit(&mut self, limit: NonZeroUsize) -> &mut Self {
        self.limit = Some(limit);

        self
    }
}

pub enum UpdateQuery<'a, T: Entity> {
    State(State),
    Update(&'a T),
    UpdateOrInsert(&'a T),
}
