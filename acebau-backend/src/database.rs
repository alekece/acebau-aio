use sqlx::{pool::PoolOptions, Database, Executor, Pool, Postgres, Transaction};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

pub enum ConnectionOptions {
    Postgres(PoolOptions<Postgres>),
}


pub enum Backend {
    Postgres(Pool<Postgres>),
}

impl Backend {
    pub fn connect(url: Url, options: ConnectionOptions) -> Result<Self, Error> {
        let backend = match options {
            ConnectionOptions::Postgres(options) => Backend::Postgres(options.connect_lazy(url.as_str())?),
        };

        Ok(backend)
    }



    // pub async fn transaction<F, Fut, R, E, DB>(&self, f: F) -> Result<R, Error>
    // where
    //     DB: Database,
    //     E: for<'a> Executor<'a, Database = DB>,
    //     Fut: std::future::Future<Output = Result<R, Error>>,
    //     F: FnOnce(&mut E) -> Fut,
    // {
    //     let transaction = match self {
    //         Backend::Postgres(pool) => pool.begin().await? as &dyn Executor<'_, Database = Postgres>,
    //     };

    //     let result = f(transaction).await?;

    //     transaction.commit().await?;
    //     return result;
    // }
    // pub async fn create<T>(&self) -> Result<Uuid, Error> {
    //     todo!()
    // }
}
