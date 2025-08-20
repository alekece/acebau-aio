use snafu::{ResultExt, Snafu};
use sqlx::{types::Json, FromRow};
use uuid::Uuid;

use crate::{database::DatabaseHandle, Executor, Record};

#[derive(Debug, Snafu)]
pub enum ProductError {
    #[snafu(display("Product '{id}' not found"))]
    NotFound {
        id: Uuid,
        source: sqlx::Error,
    },
    Insert {
        source: sqlx::Error,
    },
}

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Product {
    pub code: String,
    pub description: Option<String>,
    pub version: i32,
}

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProductVariant {
    pub product_id: Uuid,
    pub sku: String,
    pub name: Option<String>,
    pub price_ht: f32,
    pub vat_ratio: f32,
    pub resale_coefficient: f32,
    pub options: Json<Vec<ProductOption>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProductOption {
    pub name: String,
    pub value: String,
}

pub trait ProductRepository {
    fn insert_product(&mut self, product: Product) -> impl Future<Output = Result<Record<Product>, ProductError>>;
    fn fetch_product_by_id(&mut self, id: Uuid) -> impl Future<Output = Result<Record<Product>, ProductError>>;
}

impl<T> ProductRepository for DatabaseHandle<T>
where
    Self: for<'a> Executor<'a>,
{
    async fn insert_product(&mut self, product: Product) -> Result<Record<Product>, ProductError> {
        sqlx::query_as("INSERT INTO products (code, description, version) VALUES ($1, $2, $3) RETURNING *")
            .bind(product.code)
            .bind(product.description)
            .bind(product.version)
            .fetch_one(self.executor())
            .await
            .context(InsertSnafu)
    }

    async fn fetch_product_by_id(&mut self, id: Uuid) -> Result<Record<Product>, ProductError> {
        sqlx::query_as("SELECT * FROM products WHERE id = $1")
            .bind(id)
            .fetch_one(self.executor())
            .await
            .context(NotFoundSnafu { id })
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    use crate::test::UuidExt;

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("products")))]
    #[ignore = "requires a running PostgreSQL instance"]
    async fn test_find_product_by_id(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut database = DatabaseHandle::new(pool);

        let product_id = Uuid::new_fake(0);
        let product = database.fetch_product_by_id(product_id).await?;

        assert_eq!(&product.code, "P001");
        assert_eq!(product.description.as_deref(), Some("Product 1"));
        assert_eq!(product.version, 1);

        Ok(())
    }
}
