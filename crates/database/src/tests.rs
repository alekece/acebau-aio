use std::future::Future;

use sqlx::FromRow;

use crate::{DatabaseHandle, Executor, Repository, RepositoryError, Table};

trait RepositoryExt<T> {
    fn create_table(&mut self) -> impl Future<Output = Result<(), RepositoryError>>;
    fn drop_table(&mut self) -> impl Future<Output = Result<(), RepositoryError>>;
}

impl<T> RepositoryExt<Dummy> for DatabaseHandle<T>
where
    Self: for<'a> Executor<'a>,
{
    async fn create_table(&mut self) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
                create table if not exists dummies (
                    id uuid primary key default gen_random_uuid(),
                    status status not null,
                    name text not null unique,
                    age int not null,
                    created_at timestamptz not null default now(),
                    updated_at timestamptz not null default now()
                )
            "#,
        )
        .execute(self.executor())
        .await?;

        Ok(())
    }

    async fn drop_table(&mut self) -> Result<(), RepositoryError> {
        sqlx::query("drop table if exists dummies")
            .execute(self.executor())
            .await?;

        Ok(())
    }
}


#[derive(Debug, Clone, FromRow, Table)]
#[table(name = "dummies")]
#[changeset(setter(prefix = "with"))]
pub struct Dummy {
    #[table(skip)]
    pub name: String,
    pub age: i32,
}

impl Dummy {
    pub fn new(name: impl Into<String>, age: i32) -> Self {
        Self { name: name.into(), age }
    }
}

pub async fn with_setup<T>(
    pool: sqlx::PgPool,
    test: impl AsyncFnOnce(&mut DatabaseHandle<sqlx::PgPool>) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>>
where
    DatabaseHandle<sqlx::PgPool>: Repository<T>,
{
    let mut database = DatabaseHandle::new(pool);

    database.create_table().await?;

    test(&mut database).await?;

    database.drop_table().await?;

    Ok(())
}
