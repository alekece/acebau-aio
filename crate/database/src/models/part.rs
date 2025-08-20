use snafu::{ResultExt, Snafu};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{database::DatabaseHandle, types::{Duration, Quantity}, Executor, Record};

#[derive(Debug, Snafu)]
pub enum PartError {
    #[snafu(display("Part '{id}' not found: {source}"))]
    NotFound { id: Uuid, source: sqlx::Error },
    #[snafu(display("Failed to insert part: {source}"))]
    Insert { source: sqlx::Error },
    #[snafu(display("Unsupported duration"))]
    UnsupportedDuration,
}

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Part {
    pub name: String,
    pub print_duration: Duration,
    pub filament_required: Quantity,
}

pub trait PartRepository {
    fn insert_part(&mut self, part: Part) -> impl Future<Output = Result<Record<Part>, PartError>>;
    fn fetch_part_by_id(&mut self, id: Uuid) -> impl Future<Output = Result<Record<Part>, PartError>>;
}

impl<T> PartRepository for DatabaseHandle<T>
where
    Self: for<'a> Executor<'a>,
{
    async fn insert_part(&mut self, part: Part) -> Result<Record<Part>, PartError> {
        sqlx::query_as("INSERT INTO parts (name, print_duration, filament_required) VALUES ($1, $2, $3) RETURNING *")
            .bind(part.name)
            .bind(part.print_duration)
            .bind(part.filament_required)
            .fetch_one(self.executor())
            .await
            .context(InsertSnafu)
    }

    async fn fetch_part_by_id(&mut self, id: Uuid) -> Result<Record<Part>, PartError> {
        sqlx::query_as("SELECT * FROM parts WHERE id = $1")
            .bind(id)
            .fetch_one(self.executor())
            .await
            .context(NotFoundSnafu { id })
    }
}

// #[cfg(test)]
// mod tests {
//     use sqlx::PgPool;

//     use super::*;

//     use crate::test::UuidExt;

//     #[sqlx::test(fixtures(path = "../../fixtures", scripts("parts")))]
//     #[ignore = "requires a running PostgreSQL instance"]
//     async fn test_find_product_by_id(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
//         let mut database = DatabaseHandle::new(pool);

//         let product_id = Uuid::new_fake(0);
//         let part = database.fetch_part_by_id(product_id).await?;

//         assert_eq!(&part.code, "P001");
//         assert_eq!(part.description.as_deref(), Some("Part 1"));
//         assert_eq!(part.version, 1);

//         Ok(())
//     }
// }
