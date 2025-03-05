use std::{cmp::Ordering, marker::PhantomData};

use async_trait::async_trait;
use sqlx::{
    types::chrono::{DateTime, Utc},
    ColumnIndex, Database, Decode, Encode, IntoArguments, QueryBuilder, Row, Type,
};

use super::SqlExecutor;
use crate::{
    collection::{Metadata, Search},
    format::Serializable,
    record::{Record, ToRecord},
    types::{Generate, State},
    Collection, Entity, Result, Schema, SearchQuery, UpdateQuery,
};

impl<T, R> ToRecord<T, Metadata> for R
where
    R: Row + Send + Sync + Unpin + 'static,
    T: Entity,
    usize: ColumnIndex<R>,
    T::Id: for<'a> Decode<'a, R::Database> + Type<R::Database>,
    State: for<'a> Decode<'a, R::Database> + Type<R::Database>,
    for<'a> &'a str: Decode<'a, R::Database> + Type<R::Database>,
    Vec<String>: for<'a> Decode<'a, R::Database> + Type<R::Database>,
{
    fn to_record(&self) -> Result<Record<T, Metadata>> {
        Ok(Record {
            id: self.try_get(0)?,
            entity: <T as Entity>::Format::deserialize(self.try_get(1)?)?,
            metadata: Metadata {
                state: self.try_get(2)?,
                keywords: self.try_get(3)?,
            },
        })
    }
}

pub struct SqlCollection<T, DB> {
    _marker: PhantomData<(T, DB)>,
}

impl<T, DB> Default for SqlCollection<T, DB> {
    fn default() -> Self {
        Self { _marker: PhantomData }
    }
}

#[async_trait(?Send)]
impl<T, C, DB> Schema<T, C> for SqlCollection<T, DB>
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
                        state SMALLINT NOT NULL,
                        keywords TEXT[] NOT NULL
                    )
                "#,
                T::schema_name()
            ),
            format!("CREATE INDEX IF NOT EXISTS {0}_id ON {0} (id)", T::schema_name()),
            format!(
                "CREATE INDEX IF NOT EXISTS {0}_keywords ON {0} USING GIN (keywords)",
                T::schema_name()
            ),
        ];

        for query in queries {
            sqlx::query(&query).execute(connection.executor()).await?;
        }

        Ok(())
    }

    async fn drop_schema(&self, connection: &mut C) -> Result<()> {
        sqlx::query(&format!("DROP TABLE IF EXISTS {}", T::schema_name()))
            .execute(connection.executor())
            .await?;

        Ok(())
    }
}

#[async_trait(?Send)]
impl<T, C, DB> Collection<T, C> for SqlCollection<T, DB>
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
    // decode bounds
    usize: ColumnIndex<DB::Row>,
    DateTime<Utc>: for<'a> Decode<'a, DB> + Type<DB>,
    State: for<'a> Decode<'a, DB> + Type<DB>,
    for<'a> &'a str: Decode<'a, DB> + Type<DB>,
    T::Id: for<'a> Decode<'a, DB> + Type<DB>,
    Vec<String>: for<'a> Decode<'a, DB> + Type<DB>,
{
    async fn insert(&self, connection: &mut C, entity: &T) -> Result<T::Id> {
        let id = T::Id::generate();

        sqlx::query(&format!(
            "INSERT INTO {} (id, entity, state, keywords) VALUES ($1, $2, $3, $4, $5)",
            T::schema_name()
        ))
        .bind(&id)
        .bind(T::Format::serialize(entity)?)
        .bind(State::Active)
        .bind(Vec::default())
        .execute(connection.executor())
        .await?;

        Ok(id)
    }

    async fn fetch_one(&self, connection: &mut C, id: &T::Id) -> Result<Record<T, Metadata>> {
        sqlx::query(&format!(
            "SELECT id, entity, state, keywords from {} WHERE id = $1 AND state = $2",
            T::schema_name()
        ))
        .bind(id)
        .bind(State::Active)
        .fetch_one(connection.executor())
        .await
        .map_err(Into::into)
        .and_then(|row| row.to_record())
    }

    async fn fetch_many(&self, connection: &mut C, ids: &[T::Id]) -> Result<Vec<Record<T, Metadata>>> {
        sqlx::query(&format!(
            "SELECT id, entity, state, keywords from {} WHERE id IN ($1) and state = $2",
            T::schema_name()
        ))
        .bind(ids)
        .bind(State::Active)
        .fetch_all(connection.executor())
        .await?
        .into_iter()
        .map(|row| row.to_record())
        .collect::<Result<Vec<_>>>()
    }

    async fn fetch_all(&self, connection: &mut C) -> Result<Vec<Record<T, Metadata>>> {
        sqlx::query(&format!(
            "SELECT id, entity, state, keywords FROM {} WHERE state = $1",
            T::schema_name()
        ))
        .bind(State::Active)
        .fetch_all(connection.executor())
        .await?
        .into_iter()
        .map(|row| row.to_record())
        .collect::<Result<Vec<_>>>()
    }

    // async fn find<'a>(&self, connection: &mut C, query: SearchQuery<'a, T>) -> Result<Vec<Record<T>>> {
    //     // an empty search query is equivalent to fetching all records
    //     if query.is_empty() {
    //         return self.fetch_all(connection).await;
    //     }

    //     let mut query_builder = QueryBuilder::<DB>::new("SELECT ");

    //     query_builder
    //         .push(Record::<T>::select_fields())
    //         .push(" FROM ")
    //         .push(T::collection_name())
    //         .push(" WHERE ");

    //     if let Some(search) = query.state {
    //         let (states, op) = match search {
    //             Search::All(states) => (states, "AND"),
    //             Search::Any(states) => (states, "OR"),
    //             Search::EqualTo(state) => ([state].as_slice(), ""),
    //         };

    //         for (i, state) in states.into_iter().enumerate() {
    //             if i > 0 {
    //                 query_builder.push(" ").push(op).push(" ");
    //             }

    //             query_builder.push("state = ").push_bind(state);
    //         }
    //     }

    //     if let Some(search) = query.keywords {

    //     }

    //     if DB::NAME == "postgresql" {
    //         query_builder.push(" AND keywords @> ARRAY[");
    //     } else {
    //         for keyword in query.keywords {
    //             query_builder
    //                 .push(" AND keywords LIKE '%,")
    //                 .push_bind(keyword)
    //                 .push(",%'");
    //         }
    //     }

    //     let sql = query_builder.sql();

    //     sqlx::query(sql)
    //         .fetch_all(connection.executor())
    //         .await?
    //         .into_iter()
    //         .map(|row| row.to_record())
    //         .collect::<Result<Vec<_>>>()
    // }

    async fn delete(&self, connection: &mut C, id: &T::Id) -> Result<()> {
        sqlx::query(&format!("DELETE FROM {} WHERE id = $1", T::schema_name()))
            .bind(id)
            .execute(connection.executor())
            .await?;

        Ok(())
    }
    async fn delete_all(&self, connection: &mut C) -> Result<()> {
        sqlx::query(&format!("DELETE FROM {}", T::schema_name()))
            .execute(connection.executor())
            .await?;

        Ok(())
    }

    async fn update<'a>(&self, connection: &mut C, id: &T::Id, query: UpdateQuery<'a, T>) -> Result<()> {
        match query {
            UpdateQuery::State(state) => {
                sqlx::query(&format!("UPDATE {} SET state = $1 WHERE id = $2", T::schema_name()))
                    .bind(state)
                    .bind(id)
                    .execute(connection.executor())
                    .await?
            }
            UpdateQuery::Update(entity) => {
                sqlx::query(&format!("UPDATE {} SET entity = $1 WHERE id = $2", T::schema_name()))
                    .bind(T::Format::serialize(entity)?)
                    .bind(id)
                    .execute(connection.executor())
                    .await?
            }
            UpdateQuery::UpdateOrInsert(entity) => {
                sqlx::query(&format!(
                    r#"
                        INSERT INTO {} (id, entity, state, keywords)
                        VALUES ($1, $2, $3, $4)
                        ON CONFLICT(id) DO UPDATE SET entity = $5
                    "#,
                    T::schema_name()
                ))
                .bind(id)
                .bind(T::Format::serialize(entity)?)
                .bind(State::Active)
                .bind(Vec::default())
                .execute(connection.executor())
                .await?
            }
        };

        Ok(())
    }
}
