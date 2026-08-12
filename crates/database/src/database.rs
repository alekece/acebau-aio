use std::path::Path;

use snafu::{ResultExt, Snafu};
use sqlx::{PgConnection, PgPool, Postgres, migrate::MigrateError, postgres::PgPoolOptions};
use tokio::process::Command;
use url::Url;

use crate::{Repository, repository::RepositoryHandle};

#[derive(Debug, Snafu)]
pub enum DatabaseError {
    #[snafu(display("cannot connect to the database: {source}"))]
    Connection { source: sqlx::Error },
    #[snafu(display("cannot start database transaction: {source}"))]
    BeginTransaction { source: sqlx::Error },
    #[snafu(display("cannot commit transaction: {source}"))]
    CommitTransaction { source: sqlx::Error },
    #[snafu(display("cannot rollback transaction: {source}"))]
    RollbackTransaction { source: sqlx::Error },
    #[snafu(display("cannot perform migration: {source}"))]
    Migration { source: MigrateError },
    #[snafu(display("database operation failed: {source}"))]
    Internal { source: sqlx::Error },
    #[snafu(display("cannot dump database: {reason}"))]
    Dump { reason: String },
    #[snafu(display("cannot restore database: {reason}"))]
    Restore { reason: String },
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
    url: Option<Url>,
}

#[derive(Debug, Clone, Copy)]
pub struct MigrationOptions {
    /// Ignore migrations recorded in the database but not provided by this migrator.
    pub ignore_unrecognized_migrations: bool,
}

impl Default for MigrationOptions {
    fn default() -> Self {
        Self {
            ignore_unrecognized_migrations: true,
        }
    }
}

impl MigrationOptions {
    pub fn reject_unrecognized_migrations(mut self) -> Self {
        self.ignore_unrecognized_migrations = false;

        self
    }
}

impl<T> DatabaseHandle<T> {
    pub fn new(executor: T) -> Self {
        Self { executor, url: None }
    }
}

impl<T> DatabaseHandle<T> {
    pub fn repository<U>(&mut self) -> RepositoryHandle<'_, T, U>
    where
        Self: Repository<U>,
        U: async_graphql::ObjectType + async_graphql::TypeName,
    {
        RepositoryHandle::new(self)
    }
}

impl Database {
    pub async fn connect(url: Url) -> Result<Self, DatabaseError> {
        let executor = PgPoolOptions::default()
            .connect(url.as_str())
            .await
            .context(ConnectionSnafu)?;

        Ok(Self {
            executor,
            url: Some(url),
        })
    }

    pub async fn dump(&self, output: &Path, force: bool) -> Result<(), DatabaseError> {
        if output.exists() && !force {
            return Err(DatabaseError::Dump {
                reason: format!("target {} already exists", output.display()),
            });
        }

        if let Some(parent) = output.parent()
            && !parent.as_os_str().is_empty()
            && !parent.is_dir()
        {
            return Err(DatabaseError::Dump {
                reason: format!("directory {} does not exist", parent.display()),
            });
        }

        let status = Command::new("pg_dump")
            .arg("--format=custom")
            .arg("--no-owner")
            .arg("--no-privileges")
            .arg("--file")
            .arg(&output)
            .env("PGDATABASE", self.url()?.as_str())
            .status()
            .await
            .map_err(|source| DatabaseError::Dump {
                reason: source.to_string(),
            })?;

        if status.success() {
            Ok(())
        } else {
            Err(DatabaseError::Dump {
                reason: format!("operation failed with {status}"),
            })
        }
    }

    pub async fn restore(&self, input: &Path) -> Result<(), DatabaseError> {
        if !input.is_file() {
            return Err(DatabaseError::Restore {
                reason: format!("file {} does not exist", input.display()),
            });
        }

        let status = Command::new("pg_restore")
            .arg("--clean")
            .arg("--if-exists")
            .arg("--no-owner")
            .arg("--no-privileges")
            .arg("--dbname")
            .arg(self.url()?.as_str())
            .arg(&input)
            .status()
            .await
            .map_err(|source| DatabaseError::Restore {
                reason: source.to_string(),
            })?;

        if status.success() {
            Ok(())
        } else {
            Err(DatabaseError::Restore {
                reason: format!("operation failed with {status}"),
            })
        }
    }

    fn url(&self) -> Result<&Url, DatabaseError> {
        self.url.as_ref().ok_or_else(|| DatabaseError::Internal {
            source: sqlx::Error::Configuration("database connection URL is unavailable".into()),
        })
    }

    /// Runs each migrator in slice order with the same migration options.
    pub async fn migrate(
        &self,
        migrators: &[&sqlx::migrate::Migrator],
        options: MigrationOptions,
    ) -> Result<(), DatabaseError> {
        for migrator in migrators {
            let migrator = sqlx::migrate::Migrator {
                migrations: migrator.migrations.clone(),
                ignore_missing: options.ignore_unrecognized_migrations,
                locking: migrator.locking,
                no_tx: migrator.no_tx,
            };
            migrator.run(&self.executor).await.context(MigrationSnafu)?;
        }

        Ok(())
    }

    pub async fn transaction<'a>(&self) -> Result<Transaction<'a>, DatabaseError> {
        Ok(Transaction::new(
            self.executor.begin().await.context(BeginTransactionSnafu)?,
        ))
    }

    /// Removes all application rows while preserving the public schema and migration history.
    ///
    /// This is intentionally exposed only as an explicit administrative primitive;
    /// normal application startup and migrations never call it.
    pub async fn clear_all(&self) -> Result<(), DatabaseError> {
        sqlx::raw_sql(
            r#"
            do $$
            declare
                tables text;
            begin
                select string_agg(format('%I.%I', schemaname, tablename), ', ')
                into tables
                from pg_tables
                where schemaname = 'public'
                  and tablename <> '_sqlx_migrations';

                if tables is not null then
                    execute 'truncate table ' || tables || ' restart identity cascade';
                end if;
            end
            $$;
            "#,
        )
        .execute(&self.executor)
        .await
        .context(InternalSnafu)?;

        Ok(())
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
