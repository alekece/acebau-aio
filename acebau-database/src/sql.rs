//! Generic SQL database

#[cfg(feature = "postgres")]
pub mod postgres;
pub mod collection;
pub mod queue;

use std::marker::PhantomData;

use async_trait::async_trait;

use crate::{Database, Error, Result, Transaction};

pub use collection::SqlCollection;
pub use queue::SqlQueue;

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::InternalError(Box::new(e)),
        }
    }
}

/// A trait for types that can execute SQL queries.
pub trait SqlExecutor<'a, DB>
where
    DB: sqlx::Database,
{
    /// The type of the executor.
    type Executor: sqlx::Executor<'a, Database = DB>;

    /// Get the executor.
    fn executor(&'a mut self) -> Self::Executor;
}

impl<'a, DB> SqlExecutor<'a, DB> for SqlDatabase<sqlx::Pool<DB>, DB>
where
    DB: sqlx::Database,
    &'a sqlx::Pool<DB>: sqlx::Executor<'a, Database = DB>,
{
    type Executor = &'a sqlx::Pool<DB>;

    fn executor(&'a mut self) -> Self::Executor {
        &self.inner
    }
}

impl<'a, DB> SqlExecutor<'a, DB> for SqlDatabase<sqlx::Transaction<'a, DB>, DB>
where
    DB: sqlx::Database,
    &'a mut DB::Connection: sqlx::Executor<'a, Database = DB>,
{
    type Executor = &'a mut DB::Connection;

    fn executor(&'a mut self) -> Self::Executor {
        &mut *self.inner
    }
}

/// A SQL database.
/// This is a wrapper around a SQLx pool or transaction.
/// It is used to implement the `Database` trait for SQLx types.
pub struct SqlDatabase<T, DB>
where
    DB: sqlx::Database,
{
    /// The inner pool or transaction.
    inner: T,
    _marker: PhantomData<DB>,
}

impl<T, DB> SqlDatabase<T, DB>
where
    DB: sqlx::Database,
{
    /// Create a new `SqlDatabase`.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<DB> Database for SqlDatabase<sqlx::Pool<DB>, DB>
where
    Self: for<'a> SqlExecutor<'a, DB>,
    DB: sqlx::Database,
{
    type Transaction = SqlDatabase<sqlx::Transaction<'static, DB>, DB>;

    #[must_use]
    async fn begin_transaction(&self) -> Result<Self::Transaction> {
        let transaction = self.inner.begin().await?;

        Ok(SqlDatabase::new(transaction))
    }
}

#[async_trait(?Send)]
impl<DB> Transaction for SqlDatabase<sqlx::Transaction<'_, DB>, DB>
where
    DB: sqlx::Database,
{
    async fn commit(mut self) -> Result<()> {
        Ok(self.inner.commit().await?)
    }

    async fn rollback(mut self) -> Result<()> {
        Ok(self.inner.rollback().await?)
    }
}
