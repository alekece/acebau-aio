use std::num::NonZeroUsize;

use async_trait::async_trait;
use sqlx::{
    types::chrono::{DateTime, Utc},
    ColumnIndex, Database, Decode, Encode, IntoArguments, QueryBuilder, Row, Type,
};

use super::SqlExecutor;
use crate::{
    format::Serializable,
    queue::Metadata,
    record::{Record, ToRecord},
    types::{Generate, State},
    Entity, PersistenceMode, PullOptions, PullStrategy, Queue, Result, Schema,
};

impl<T, R> ToRecord<T, Metadata> for R
where
    R: Row + Send + Sync + Unpin + 'static,
    T: Entity,
    usize: ColumnIndex<R>,
    T::Id: for<'a> Decode<'a, R::Database> + Type<R::Database>,
    DateTime<Utc>: for<'a> Decode<'a, R::Database> + Type<R::Database>,
    for<'a> &'a str: Decode<'a, R::Database> + Type<R::Database>,
{
    fn to_record(&self) -> Result<Record<T, Metadata>> {
        Ok(Record {
            id: self.try_get(0)?,
            entity: <T as Entity>::Format::deserialize(self.try_get(1)?)?,
            metadata: Metadata {
                created_at: self.try_get(2)?,
            },
        })
    }
}

pub struct SqlQueue<T: Entity, DB> {
    name: String,
    _marker: std::marker::PhantomData<(T, DB)>,
}

impl<T: Entity, DB> SqlQueue<T, DB> {
    pub fn new(suffix: &str) -> Self {
        Self {
            name: format!("q_{}_{}", T::schema_name(), suffix),
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<T, C, DB> Schema<T, C> for SqlQueue<T, DB>
where
    T: Entity + 'static,
    DB: Database,
    C: for<'a> SqlExecutor<'a, DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
{
    async fn create_schema(&self, connection: &mut C) -> Result<()> {
        let queries = [
            format!(
                r#"
                    CREATE TABLE IF NOT EXISTS {} (
                        id UUID PRIMARY KEY NOT NULL,
                        entity TEXT NOT NULL,
                        created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                        pulled BOOLEAN NOT NULL
                    )
                "#,
                self.name
            ),
            format!(
                "CREATE INDEX IF NOT EXISTS {0}_created_at_idx ON {0}(created_at)",
                self.name
            ),
        ];

        for query in queries {
            sqlx::query(&query).execute(connection.executor()).await?;
        }

        Ok(())
    }

    async fn drop_schema(&self, connection: &mut C) -> Result<()> {
        sqlx::query(&format!("DROP TABLE IF EXISTS {}", self.name))
            .execute(connection.executor())
            .await?;

        Ok(())
    }
}

#[async_trait(?Send)]
impl<T, C, DB> Queue<T, C> for SqlQueue<T, DB>
where
    T: Entity + 'static,
    DB: Database,
    C: for<'a> SqlExecutor<'a, DB>,
    // encode bounds
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    for<'a> &'a str: Encode<'a, DB> + Type<DB>,
    T::Id: for<'a> Encode<'a, DB> + Type<DB>,
    for<'a> &'a [T::Id]: Encode<'a, DB> + Type<DB>,
    State: for<'a> Encode<'a, DB> + Type<DB>,
    DateTime<Utc>: for<'a> Encode<'a, DB> + Type<DB>,
    String: for<'a> Encode<'a, DB> + Type<DB>,
    Vec<String>: for<'a> Encode<'a, DB> + Type<DB>,
    bool: for<'a> Encode<'a, DB> + Type<DB>,
    // decode bounds
    usize: ColumnIndex<DB::Row>,
    DateTime<Utc>: for<'a> Decode<'a, DB> + Type<DB>,
    State: for<'a> Decode<'a, DB> + Type<DB>,
    for<'a> &'a str: Decode<'a, DB> + Type<DB>,
    T::Id: for<'a> Decode<'a, DB> + Type<DB>,
    Vec<String>: for<'a> Decode<'a, DB> + Type<DB>,
    i64: for<'a> Decode<'a, DB> + Type<DB>,
{
    async fn push(&self, connection: &mut C, entity: &T) -> Result<T::Id> {
        let id = T::Id::generate();

        sqlx::query(&format!(
            "INSERT INTO {} (id, entity, created_at, pulled) VALUES($1, $2, $3, $4)",
            self.name
        ))
        .bind(&id)
        .bind(T::Format::serialize(entity)?)
        .bind(Utc::now())
        .bind(false)
        .execute(connection.executor())
        .await?;

        Ok(id)
    }

    async fn pull(&self, connection: &mut C, options: PullOptions) -> Result<Option<Record<T, Metadata>>> {
        Ok(self
            .pull_many(connection, options, NonZeroUsize::new(1).unwrap())
            .await?
            .pop())
    }

    async fn pull_many(
        &self,
        connection: &mut C,
        options: PullOptions,
        limit: NonZeroUsize,
    ) -> Result<Vec<Record<T, Metadata>>> {
        let mut query_builder = QueryBuilder::<DB>::new("SELECT id FROM ");

        query_builder.push(&self.name).push(" WHERE pulled = FALSE ORDER BY ");

        match options.pull_strategy {
            PullStrategy::FirstInFirstOut => query_builder.push("created_at ASC"),
            PullStrategy::LastInFirstOut => query_builder.push("created_at DESC"),
            PullStrategy::Random => query_builder.push("RANDOM()"),
        };

        query_builder
            .push(" FOR UPDATE SKIP LOCKED")
            .push(" LIMIT ")
            .push(limit.get());

        let query = match options.persistence_mode {
            PersistenceMode::Keep => format!(
                "UPDATE {} SET pulled = TRUE WHERE id IN ({}) RETURNING id, entity, created_at",
                self.name,
                query_builder.sql()
            ),
            PersistenceMode::Remove => format!(
                "DELETE FROM {} WHERE id IN ({}) RETURNING id, entity, created_at",
                self.name,
                query_builder.sql()
            ),
        };

        sqlx::query(&query)
            .fetch_all(connection.executor())
            .await?
            .into_iter()
            .map(|row| row.to_record())
            .collect::<Result<Vec<_>>>()
    }

    async fn delete(&self, connection: &mut C, id: T::Id) -> Result<()> {
        sqlx::query(&format!("DELETE FROM {} WHERE id = $1", self.name))
            .bind(&id)
            .execute(connection.executor())
            .await?;

        Ok(())
    }

    async fn clear_pulled(&self, connection: &mut C) -> Result<()> {
        sqlx::query(&format!("DELETE FROM {} WHERE pulled = TRUE", self.name))
            .execute(connection.executor())
            .await?;

        Ok(())
    }

    async fn len(&self, connection: &mut C) -> Result<usize> {
        let len: i64 = sqlx::query(&format!("SELECT COUNT(*) FROM {}", self.name))
            .fetch_one(connection.executor())
            .await?
            .try_get(0)?;

        Ok(len as usize)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use sqlx::{Database, Pool, Postgres};

    use crate::{format::Json, sql::SqlDatabase, types::OrderedId};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: i16,
    }

    impl Person {
        pub fn new(name: &str, age: i16) -> Self {
            Self {
                name: name.to_string(),
                age,
            }
        }
    }

    impl Entity for Person {
        type Id = OrderedId;
        type Format = Json;

        fn schema_name() -> &'static str {
            "persons"
        }
    }

    async fn test_with<T, DB>(
        url: &str,
        f: impl AsyncFn(SqlDatabase<Pool<DB>, DB>, SqlQueue<T, DB>) -> Result<()>,
    ) -> Result<()>
    where
        T: Entity,
        DB: Database,
        SqlQueue<T, DB>: Queue<T, SqlDatabase<Pool<DB>, DB>>,
    {
        let mut database = SqlDatabase::new(Pool::<DB>::connect(url).await?);
        let queue = SqlQueue::<T, DB>::new("test");

        queue.drop_schema(&mut database).await?;
        queue.create_schema(&mut database).await?;

        f(database, queue).await?;

        Ok(())
    }

    #[tokio::test]
    async fn queue_pull_first_item() -> Result<()> {
        test_with::<Person, Postgres>(
            "postgres://postgres:password@localhost:5432",
            async |mut database, queue| {
                let pull_options = PullOptions::default().fifo().keep();

                queue.push(&mut database, &Person::new("Alice", 20)).await?;
                queue.push(&mut database, &Person::new("Bob", 30)).await?;

                let person = queue.pull(&mut database, pull_options).await?;

                assert!(person.is_some());
                assert_eq!(person.unwrap().name, "Alice");

                let person = queue.pull(&mut database, pull_options).await?;

                assert!(person.is_some());
                assert_eq!(person.unwrap().name, "Bob");

                assert_eq!(queue.len(&mut database).await?, 2);

                queue.clear_pulled(&mut database).await?;

                assert_eq!(queue.len(&mut database).await?, 0);

                Ok(())
            },
        )
        .await?;

        Ok(())
    }
}
//     #[tes]
//         let database = PgDatabase::new(PgPool::connect("postgres://postgres:password@localhost:5432").await?);
//         let mut database = database.begin_transaction().await?;
//         let persons = PgCollection::<Person>::default();

//         persons.schema_mut().drop_collection().await?;

//         assert!(!persons.collection_exists().await?);

//         persons.create_collection().await?;

//         assert!(persons.collection_exists().await?);
//         assert!(persons.fetch_all().await?.is_empty());

//         let mut person = Person {
//             name: "Alice".to_string(),
//             age: 20,
//         };

//         let uuid = persons.insert(&person).await?;
//         let fetched_person = persons.fetch_one(&uuid).await?;
//         assert_eq!(fetched_person.name, person.name);
//         assert_eq!(fetched_person.age, person.age);

//         person.age = 42;
//         persons.update(&uuid, UpdateQuery::Update(&person)).await?;
//         let fetched_person = persons.fetch_one(&uuid).await?;
//         assert_eq!(fetched_person.age, 42);

//         persons.update(&uuid, UpdateQuery::State(State::Inactive)).await?;
//         assert!(persons.fetch_all().await?.is_empty());

//         let person = Person {
//             name: "Bob".to_string(),
//             age: 30,
//         };
//         let uuid = OrderedId::generate();
//         persons.update(&uuid, UpdateQuery::UpdateOrInsert(&person)).await?;
//         let fetched_person = persons.fetch_one(&uuid).await?;
//         assert_eq!(fetched_person.name, person.name);
//         assert_eq!(fetched_person.age, person.age);

//         Ok(())
//     }
// }
