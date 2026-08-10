use std::{collections::HashSet, marker::PhantomData};

use snafu::Snafu;
use uuid::Uuid;

use crate::{types::Status, Record};

#[derive(Debug, Snafu)]
pub enum RepositoryError {
    #[snafu(display("Item `{id}` not found"))]
    NotFound { id: Uuid },
    #[snafu(display("No changes provided for item `{id}`"))]
    NoChanges { id: Uuid },
    #[snafu(display("Cannot duplicate item: {source}"))]
    Duplicate {
        source: Box<dyn sqlx::error::DatabaseError>,
    },
    #[snafu(display("Internal error: {source}"))]
    Internal { source: sqlx::Error },
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(error) if error.is_unique_violation() => Self::Duplicate { source: error },
            _ => Self::Internal { source: error },
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchOptions {
    statuses: HashSet<Status>,
}

impl FetchOptions {
    pub fn active_only() -> Self {
        Self {
            statuses: HashSet::from([Status::Active]),
        }
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.statuses.insert(status);

        self
    }

    pub fn statuses(&self) -> impl Iterator<Item = Status> {
        self.statuses.iter().copied()
    }
}

impl Default for FetchOptions {
    fn default() -> Self {
        Self {
            statuses: HashSet::default(),
        }
    }
}

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

pub struct RepositoryHandle<'a, R, T> {
    repository: &'a mut R,
    _marker: PhantomData<T>,
}

impl<'a, R, T> RepositoryHandle<'a, R, T>
where
    R: Repository<T>,
{
    pub fn new(repository: &'a mut R) -> Self {
        Self {
            repository,
            _marker: PhantomData,
        }
    }
}

impl<R, T> Repository<T> for RepositoryHandle<'_, R, T>
where
    R: Repository<T>,
{
    type Error = R::Error;
    type Changeset = R::Changeset;

    async fn insert(&mut self, item: &T, status: Status) -> Result<Record<T>, Self::Error> {
        self.repository.insert(item, status).await
    }

    async fn update(
        &mut self,
        id: Uuid,
        changeset: &Self::Changeset,
        status: Option<Status>,
    ) -> Result<Record<T>, Self::Error> {
        self.repository.update(id, changeset, status).await
    }

    async fn fetch_by_id(&mut self, id: Uuid) -> Result<Record<T>, Self::Error> {
        self.repository.fetch_by_id(id).await
    }

    async fn fetch_all(&mut self, options: FetchOptions) -> Result<Vec<Record<T>>, Self::Error> {
        self.repository.fetch_all(options).await
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), Self::Error> {
        self.repository.delete(id).await
    }

    async fn status(&mut self, id: Uuid) -> Result<Status, Self::Error> {
        self.repository.status(id).await
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use uuid::Uuid;

    use crate::{
        tests::{self, Dummy, DummyChangeset},
        types::Status,
        FetchOptions, Repository, RepositoryError,
    };

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn insert(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let item = Dummy {
                name: "Alice".to_string(),
                age: 30,
            };

            let inserted_item = database.insert(&item, Status::Active).await?;

            assert_eq!(inserted_item.name, "Alice");
            assert_eq!(inserted_item.age, 30);

            let result = database.insert(&item, Status::Active).await;

            assert_matches!(result, Err(crate::RepositoryError::Duplicate { .. }));

            let item = database.insert(&Dummy::new("Bob", 20), Status::Draft).await?;

            assert_eq!(item.name, "Bob");
            assert_eq!(item.age, 20);
            assert_eq!(item.status(), Status::Draft);

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn update(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let id = database.insert(&Dummy::new("Alice", 30), Status::Active).await?.id();

            let item = database
                .repository::<Dummy>()
                .update(id, &DummyChangeset::default(), Some(Status::Draft))
                .await?;

            assert_eq!(item.status(), Status::Draft);

            let item = database
                .repository::<Dummy>()
                .update(id, &DummyChangeset::default().with_age(32), Some(Status::Archived))
                .await?;

            assert_eq!(item.status(), Status::Archived);
            assert_eq!(item.age, 32);

            let result = database
                .repository::<Dummy>()
                .update(id, &DummyChangeset::default(), None)
                .await;

            assert_matches!(result, Err(RepositoryError::NoChanges { .. }));

            Ok(())
        })
        .await
    }

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn fetch_by_id(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        tests::with_setup::<Dummy>(pool, async |database| {
            let id = database.insert(&Dummy::new("Alice", 30), Status::Active).await?.id();
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

            database.insert(&Dummy::new("Alice", 30), Status::Active).await?;
            database.insert(&Dummy::new("Bob", 25), Status::Draft).await?;
            database.insert(&Dummy::new("Charlie", 35), Status::Active).await?;

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
            let id = database.insert(&Dummy::new("Alice", 30), Status::Active).await?.id();
            database.insert(&Dummy::new("Bob", 25), Status::Active).await?;
            database.insert(&Dummy::new("Charlie", 35), Status::Active).await?;

            database.repository::<Dummy>().delete(id).await?;

            let items = database
                .repository::<Dummy>()
                .fetch_all(FetchOptions::active_only())
                .await?;

            assert_eq!(items.len(), 2);
            assert_eq!(items[0].name, "Bob");
            assert_eq!(items[1].name, "Charlie");

            database.repository::<Dummy>().delete(items[1].id()).await?;

            let items = database
                .repository::<Dummy>()
                .fetch_all(FetchOptions::active_only())
                .await?;

            assert_eq!(items.len(), 1);
            assert_eq!(items[0].name, "Bob");

            database.repository::<Dummy>().delete(items[0].id()).await?;

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
            let id = database.insert(&Dummy::new("Alice", 30), Status::Active).await?.id();

            let status = database.repository::<Dummy>().status(id).await?;
            assert_eq!(status, Status::Active);

            database
                .repository::<Dummy>()
                .update(id, &DummyChangeset::default(), Some(Status::Draft))
                .await?;

            let status = database.repository::<Dummy>().status(id).await?;
            assert_eq!(status, Status::Draft);

            database
                .repository::<Dummy>()
                .update(id, &DummyChangeset::default(), Some(Status::Archived))
                .await?;


            let status = database.repository::<Dummy>().status(id).await?;
            assert_eq!(status, Status::Archived);
           
            Ok(())
        })
        .await
    }
}
