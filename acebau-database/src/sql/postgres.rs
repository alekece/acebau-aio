use sqlx::Postgres;

use super::{SqlCollection, SqlDatabase, SqlQueue};

pub type PgDatabase<T> = SqlDatabase<T, Postgres>;
pub type PgCollection<T> = SqlCollection<T, Postgres>;
pub type PgQueue<T> = SqlQueue<T, Postgres>;

// #[cfg(test)]
// mod tests {
//     use serde::{Deserialize, Serialize};
//     use sqlx::PgPool;

//     use crate::{
//         format::Json, types::{Generate, OrderedId, State}, Database, Entity, Result, UpdateQuery
//     };

//     use super::*;

//     #[derive(Debug, Serialize, Deserialize)]
//     struct Person {
//         name: String,
//         age: i16,
//     }

//     impl Entity for Person {
//         type Id = OrderedId;
//         type Format = Json;

//         fn schema_name() -> &'static str {
//             "person"
//         }
//     }

//     #[tokio::test]
//     async fn test_crud() -> Result<()> {
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
