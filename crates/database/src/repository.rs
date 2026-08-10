use std::{collections::HashSet, marker::PhantomData, num::NonZeroU32};

use acebau_core::{InvalidPageSize, PageRequest, PageSize};
use async_graphql::{ObjectType, TypeName};
use snafu::Snafu;
use uuid::Uuid;

use crate::{DatabaseHandle, Record, Status};

#[derive(Debug, Snafu)]
pub enum RepositoryError {
    #[snafu(display("record `{id}` not found"))]
    NotFound { id: Uuid },
    #[snafu(display("no changes provided for record `{id}`"))]
    NoChanges { id: Uuid },
    #[snafu(display("cannot duplicate record: {source}"))]
    Duplicate {
        source: Box<dyn sqlx::error::DatabaseError>,
    },
    #[snafu(display("cannot perform operation due to foreign key violation: {source}"))]
    ForeignKeyViolation {
        source: Box<dyn sqlx::error::DatabaseError>,
    },
    #[snafu(display("internal error: {source}"))]
    Internal { source: sqlx::Error },
    #[snafu(display("{source}"))]
    InvalidPageSize { source: InvalidPageSize },
    #[snafu(display("page number must be greater than zero"))]
    InvalidPage,
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(error) if error.is_unique_violation() => Self::Duplicate { source: error },
            sqlx::Error::Database(error) if error.is_foreign_key_violation() => {
                Self::ForeignKeyViolation { source: error }
            }
            _ => Self::Internal { source: error },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FetchOptions {
    statuses: HashSet<Status>,
    page: PageRequest,
}

impl FetchOptions {
    pub fn active_only() -> Self {
        Self {
            statuses: HashSet::from([Status::Active]),
            page: PageRequest::default(),
        }
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.statuses.insert(status);

        self
    }

    pub fn statuses(&self) -> impl Iterator<Item = Status> {
        self.statuses.iter().copied()
    }

    pub fn with_page(mut self, page: u32, page_size: u32) -> Result<Self, RepositoryError> {
        let page = NonZeroU32::new(page).ok_or(RepositoryError::InvalidPage)?;
        let size = PageSize::try_from(page_size).map_err(|source| RepositoryError::InvalidPageSize { source })?;
        self.page = PageRequest::new(page, size);
        Ok(self)
    }

    pub fn limit(&self) -> u32 {
        self.page.limit()
    }

    pub fn offset(&self) -> u32 {
        self.page.offset()
    }
}

pub trait Repository<T: ObjectType + TypeName> {
    type Error;
    type Changeset;

    fn insert(&mut self, item: &T) -> impl Future<Output = Result<Record<T>, Self::Error>> + Send;
    fn patch(
        &mut self,
        id: Uuid,
        changeset: &Self::Changeset,
    ) -> impl Future<Output = Result<Record<T>, Self::Error>> + Send;
    fn update(&mut self, id: Uuid, item: &T) -> impl Future<Output = Result<Record<T>, Self::Error>> + Send;
    fn fetch_by_id(&mut self, id: Uuid) -> impl Future<Output = Result<Record<T>, Self::Error>> + Send;
    fn fetch_all(&mut self, options: FetchOptions) -> impl Future<Output = Result<Vec<Record<T>>, Self::Error>> + Send;
    fn delete(&mut self, id: Uuid) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub struct RepositoryHandle<'a, E, T> {
    database: &'a mut DatabaseHandle<E>,
    _marker: PhantomData<T>,
}

impl<'a, E, T> RepositoryHandle<'a, E, T>
where
    DatabaseHandle<E>: Repository<T>,
    T: ObjectType + TypeName,
{
    pub fn new(database: &'a mut DatabaseHandle<E>) -> Self {
        Self {
            database,
            _marker: PhantomData,
        }
    }
}

impl<E, T> Repository<T> for RepositoryHandle<'_, E, T>
where
    E: Send,
    DatabaseHandle<E>: Repository<T>,
    T: ObjectType + TypeName + Send + Sync,
    <DatabaseHandle<E> as Repository<T>>::Changeset: Sync,
{
    type Error = <DatabaseHandle<E> as Repository<T>>::Error;
    type Changeset = <DatabaseHandle<E> as Repository<T>>::Changeset;

    async fn insert(&mut self, item: &T) -> Result<Record<T>, Self::Error> {
        self.database.insert(item).await
    }

    async fn patch(&mut self, id: Uuid, changeset: &Self::Changeset) -> Result<Record<T>, Self::Error> {
        self.database.patch(id, changeset).await
    }

    async fn update(&mut self, id: Uuid, item: &T) -> Result<Record<T>, Self::Error> {
        self.database.update(id, item).await
    }

    async fn fetch_by_id(&mut self, id: Uuid) -> Result<Record<T>, Self::Error> {
        self.database.fetch_by_id(id).await
    }

    async fn fetch_all(&mut self, options: FetchOptions) -> Result<Vec<Record<T>>, Self::Error> {
        self.database.fetch_all(options).await
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), Self::Error> {
        self.database.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use uuid::Uuid;

    use crate::{
        FetchOptions, Repository, RepositoryError, Status,
        tests::{self, Dummy, DummyChangeset},
    };

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn insert(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let item = Dummy {
                name: "Alice".to_string(),
                age: 30,
            };

            let inserted_item = database.insert(&item).await?;

            assert_eq!(inserted_item.name, "Alice");
            assert_eq!(inserted_item.age, 30);

            let result = database.insert(&item).await;

            assert_matches!(result, Err(crate::RepositoryError::Duplicate { .. }));

            let item = database.insert(&Dummy::new("Bob", 20)).await?;

            assert_eq!(item.name, "Bob");
            assert_eq!(item.age, 20);
            assert_eq!(item.status, Status::Active);

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn patch(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let id = database.insert(&Dummy::new("Alice", 30)).await?.id;

            let item = database
                .repository::<Dummy>()
                .patch(id, &DummyChangeset::default().with_status(Status::Draft))
                .await?;

            assert_eq!(item.status, Status::Draft);

            let item = database
                .repository::<Dummy>()
                .patch(
                    id,
                    &DummyChangeset::default().with_age(32).with_status(Status::Archived),
                )
                .await?;

            assert_eq!(item.status, Status::Archived);
            assert_eq!(item.age, 32);

            let result = database
                .repository::<Dummy>()
                .patch(id, &DummyChangeset::default())
                .await;

            assert_matches!(result, Err(RepositoryError::NoChanges { .. }));

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn update(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let id = database.insert(&Dummy::new("Alice", 30)).await?.id;

            let replaced = database
                .repository::<Dummy>()
                .update(id, &Dummy::new("Alice replaced", 40))
                .await?;

            assert_eq!(replaced.id, id);
            assert_eq!(replaced.name, "Alice replaced");
            assert_eq!(replaced.age, 40);
            assert_eq!(replaced.status, Status::Active);

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn fetch_by_id(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let id = database.insert(&Dummy::new("Alice", 30)).await?.id;
            let item = database.repository::<Dummy>().fetch_by_id(id).await?;

            assert_eq!(item.name, "Alice");
            assert_eq!(item.age, 30);

            let result = database.repository::<Dummy>().fetch_by_id(Uuid::new_v4()).await;

            assert_matches!(result, Err(RepositoryError::NotFound { .. }));

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn fetch_all(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let items = database
                .repository::<Dummy>()
                .fetch_all(FetchOptions::active_only())
                .await?;

            assert!(items.is_empty());

            database.insert(&Dummy::new("Alice", 30)).await?;
            let bob = database.insert(&Dummy::new("Bob", 25)).await?;
            database
                .repository::<Dummy>()
                .patch(bob.id, &DummyChangeset::default().with_status(Status::Draft))
                .await?;
            database.insert(&Dummy::new("Charlie", 35)).await?;

            let items = database
                .repository::<Dummy>()
                .fetch_all(FetchOptions::active_only())
                .await?;

            assert_eq!(items.len(), 2);
            assert_eq!(items[0].name, "Alice");
            assert_eq!(items[1].name, "Charlie");

            let items = database
                .repository::<Dummy>()
                .fetch_all(
                    FetchOptions::default()
                        .with_status(Status::Active)
                        .with_status(Status::Draft),
                )
                .await?;

            assert_eq!(items.len(), 3);
            assert_eq!(items[0].name, "Alice");
            assert_eq!(items[1].name, "Bob");
            assert_eq!(items[2].name, "Charlie");

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn delete(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let id = database.insert(&Dummy::new("Alice", 30)).await?.id;
            database.insert(&Dummy::new("Bob", 25)).await?;
            database.insert(&Dummy::new("Charlie", 35)).await?;

            database.repository::<Dummy>().delete(id).await?;

            let items = database
                .repository::<Dummy>()
                .fetch_all(FetchOptions::active_only())
                .await?;

            assert_eq!(items.len(), 2);
            assert_eq!(items[0].name, "Bob");
            assert_eq!(items[1].name, "Charlie");

            database.repository::<Dummy>().delete(items[1].id).await?;

            let items = database
                .repository::<Dummy>()
                .fetch_all(FetchOptions::active_only())
                .await?;

            assert_eq!(items.len(), 1);
            assert_eq!(items[0].name, "Bob");

            database.repository::<Dummy>().delete(items[0].id).await?;

            let items = database
                .repository::<Dummy>()
                .fetch_all(
                    FetchOptions::default()
                        .with_status(Status::Active)
                        .with_status(Status::Draft),
                )
                .await?;

            assert!(items.is_empty());

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn status(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let id = database.insert(&Dummy::new("Alice", 30)).await?.id;

            let status = database.repository::<Dummy>().fetch_by_id(id).await?.status;
            assert_eq!(status, Status::Active);

            database
                .repository::<Dummy>()
                .patch(id, &DummyChangeset::default().with_status(Status::Draft))
                .await?;

            let status = database.repository::<Dummy>().fetch_by_id(id).await?.status;
            assert_eq!(status, Status::Draft);

            database
                .repository::<Dummy>()
                .patch(id, &DummyChangeset::default().with_status(Status::Archived))
                .await?;

            let status = database.repository::<Dummy>().fetch_by_id(id).await?.status;
            assert_eq!(status, Status::Archived);

            Ok(())
        })
        .await
    }
}
