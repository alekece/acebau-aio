use snafu::{ResultExt, Snafu};
use sqlx::{Error, FromRow, QueryBuilder};
use uuid::Uuid;

use crate::{
    DatabaseHandle, Executor, FetchOptions, Record, Repository,
    types::{Percentage, Status},
};

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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrintingEnvironmentPatch {
    pub name: Option<String>,
    pub operating_factor: Option<Percentage>,
    pub electricity_cost_per_kwh: Option<f32>,
}

impl<T> Repository<PrintingEnvironment> for DatabaseHandle<T>
where
    Self: for<'a> Executor<'a>,
{
    type Error = PrintingEnvironmentError;
    type Changeset = PrintingEnvironmentPatch;

    async fn insert(
        &mut self,
        item: &PrintingEnvironment,
        status: Status,
    ) -> Result<Record<PrintingEnvironment>, Self::Error> {
        sqlx::query_as(
            "INSERT INTO printing_environments (name, operating_factor, electricity_cost_per_kwh, status) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(&item.name)
        .bind(item.operating_factor)
        .bind(item.electricity_cost_per_kwh)
        .bind(status)
        .fetch_one(self.executor())
        .await
        .context(InsertFailedSnafu)
    }

    async fn update(
        &mut self,
        id: Uuid,
        changeset: &Self::Changeset,
        status: Option<Status>,
    ) -> Result<Record<PrintingEnvironment>, Self::Error> {
        let mut query_builder = QueryBuilder::new("UPDATE printing_environments SET ");
        let mut values = query_builder.separated(", ");

        if let Some(name) = &changeset.name {
            values.push("name = ").push_bind_unseparated(name);
        }
        if let Some(operating_factor) = &changeset.operating_factor {
            values
                .push("operating_factor = ")
                .push_bind_unseparated(operating_factor);
        }
        if let Some(electricity_cost_per_kwh) = &changeset.electricity_cost_per_kwh {
            values
                .push("electricity_cost_per_kwh = ")
                .push_bind_unseparated(electricity_cost_per_kwh);
        }

        if let Some(status) = status {
            values.push("status = ").push_bind_unseparated(status);
        }

        query_builder.push(" WHERE id = ").push_bind(id).push(" RETURNING *");

        query_builder
            .build_query_as()
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

    async fn status(&mut self, id: Uuid) -> Result<Status, Self::Error> {
        sqlx::query_scalar("SELECT status FROM printing_environments WHERE id = $1")
            .bind(id)
            .fetch_one(self.executor())
            .await
            .context(NotFoundSnafu { id })
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

        let inserted_environment = database.insert(&environment, Status::Active).await?;

        assert_eq!(inserted_environment.name, "Test Environment");
        assert_eq!(inserted_environment.operating_factor.to_float(), 0.8);
        assert_eq!(inserted_environment.electricity_cost_per_kwh, 0.15);

        Ok(())
    }
}
