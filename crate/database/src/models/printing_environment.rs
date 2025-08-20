use snafu::{ResultExt, Snafu};
use sqlx::{Error, FromRow};
use uuid::Uuid;

use crate::{database::DatabaseHandle, repository::FetchOptions, types::Percentage, Executor, Record, Repository};

#[derive(Debug, Snafu)]
pub enum PrintingEnvironmentError {
    #[snafu(display("Cannot insert printing environment: {source}"))]
    InsertFailed { source: sqlx::Error },
    #[snafu(display("Cannot update printing environment '{id}': {source}"))]
    UpdateFailed { id: Uuid, source: sqlx::Error },
    #[snafu(display("Printing environment '{id}' not found: {source}"))]
    NotFound { id: Uuid, source: Error },
    #[snafu(display("Cannot fetch all printing environments: {source}"))]
    FetchAllFailed { source: Error },
}

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrintingEnvironment {
    pub name: String,
    pub operating_factor: Percentage,
    pub electricity_cost_per_kwh: f32,
}

impl<T> Repository<PrintingEnvironment> for DatabaseHandle<T>
where
    Self: for<'a> Executor<'a>,
{
    type Error = PrintingEnvironmentError;

    async fn insert(&mut self, item: PrintingEnvironment) -> Result<Record<PrintingEnvironment>, Self::Error> {
        sqlx::query_as(
            "INSERT INTO printing_environments (name, operating_factor, electricity_cost_per_kwh) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&item.name)
        .bind(item.operating_factor)
        .bind(item.electricity_cost_per_kwh)
        .fetch_one(self.executor())
        .await
        .context(InsertFailedSnafu)
    }

    async fn update(
        &mut self,
        id: Uuid,
        item: PrintingEnvironment,
    ) -> Result<Record<PrintingEnvironment>, Self::Error> {
        sqlx::query_as(
            "UPDATE printing_environments SET name = $1, operating_factor = $2, electricity_cost_per_kwh = $3 WHERE id = $4 RETURNING *",
        )
            .bind(&item.name)
            .bind(item.operating_factor)
            .bind(item.electricity_cost_per_kwh)
            .fetch_one(self.executor())
            .await
            .context(UpdateFailedSnafu { id })
    }

    async fn fetch_by_id(&mut self, id: Uuid) -> Result<Record<PrintingEnvironment>, Self::Error> {
        sqlx::query_as("SELECT * FROM printing_environments WHERE id = $1")
            .bind(id)
            .fetch_one(self.executor())
            .await
            .context(NotFoundSnafu { id })
    }

    async fn fetch_all(&mut self, options: FetchOptions) -> Result<Vec<Record<PrintingEnvironment>>, Self::Error> {
        let query = match options.status() {
            Some(status) => sqlx::query_as("SELECT * FROM printing_environments WHERE status = $1").bind(status),
            _ => sqlx::query_as("SELECT * FROM printing_environments"),
        };

        query.fetch_all(self.executor()).await.context(FetchAllFailedSnafu)
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), Self::Error> {
        sqlx::query("DELETE FROM printing_environments WHERE id = $1")
            .bind(id)
            .execute(self.executor())
            .await
            .context(NotFoundSnafu { id })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn test_printing_environment_insert(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut database = DatabaseHandle::new(pool);

        let environment = PrintingEnvironment {
            name: "Test Environment".to_string(),
            operating_factor: Percentage::try_new(80.).unwrap(),
            electricity_cost_per_kwh: 0.15,
        };

        let inserted_environment = database.insert(environment).await?;

        assert_eq!(inserted_environment.name, "Test Environment");
        assert_eq!(inserted_environment.operating_factor.to_float(), 0.8);
        assert_eq!(inserted_environment.electricity_cost_per_kwh, 0.15);

        Ok(())
    }
}
