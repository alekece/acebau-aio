use snafu::{ResultExt, Snafu};
use sqlx::{migrate::MigrateError, postgres::PgPoolOptions, PgConnection, PgPool, Postgres};
use url::Url;

#[derive(Debug, Snafu)]
pub enum DatabaseError {
    #[snafu(display("Failed to connect to the database: {source}"))]
    Connection { source: sqlx::Error },
    #[snafu(display("Failed to start database transaction: {source}"))]
    BeginTransaction { source: sqlx::Error },
    #[snafu(display("Failed to commit transaction: {source}"))]
    CommitTransaction { source: sqlx::Error },
    #[snafu(display("Failed to rollback transaction: {source}"))]
    RollbackTransaction { source: sqlx::Error },
    #[snafu(display("Migration failed: {source}"))]
    Migration { source: MigrateError },
    #[snafu(display("Database operation failed: {source}"))]
    Internal { source: sqlx::Error },
}

pub type Transaction<'a> = DatabaseHandle<sqlx::Transaction<'a, Postgres>>;
pub type Database = DatabaseHandle<PgPool>;

pub trait Executor<'a> {
    type Executor: sqlx::Executor<'a, Database = Postgres>;

    fn executor(&'a mut self) -> Self::Executor;
}

#[derive(Debug, Clone)]
pub struct DatabaseHandle<T> {
    executor: T,
}

impl<T> DatabaseHandle<T> {
    pub fn new(executor: T) -> Self {
        Self { executor }
    }
}

impl Database {
    pub async fn connect(url: Url) -> Result<Self, DatabaseError> {
        Self::connect_with(url.as_str(), PgPoolOptions::default()).await
    }

    pub async fn connect_with(url: &str, options: PgPoolOptions) -> Result<Self, DatabaseError> {
        Ok(Self::new(options.connect(url).await.context(ConnectionSnafu)?))
    }

    pub async fn transaction<'a>(&self) -> Result<Transaction<'a>, DatabaseError> {
        Ok(Transaction::new(
            self.executor.begin().await.context(BeginTransactionSnafu)?,
        ))
    }
}

impl<'a> Executor<'a> for Database {
    type Executor = &'a PgPool;

    fn executor(&'a mut self) -> Self::Executor {
        &self.executor
    }
}

impl Transaction<'_> {
    pub async fn commit(self) -> Result<(), DatabaseError> {
        self.executor.commit().await.context(CommitTransactionSnafu)
    }

    pub async fn rollback(self) -> Result<(), DatabaseError> {
        self.executor.rollback().await.context(RollbackTransactionSnafu)
    }
}

impl<'a> Executor<'a> for Transaction<'_> {
    type Executor = &'a mut PgConnection;

    fn executor(&'a mut self) -> Self::Executor {
        &mut *self.executor
    }
}
