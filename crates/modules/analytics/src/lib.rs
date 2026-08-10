#![allow(clippy::pedantic)]

use acebau_database::{Database, Executor};
use async_graphql::{Context, Object, SimpleObject};

#[derive(Debug, SimpleObject)]
pub struct OperationalOverview {
    pub active_products: i64,
    pub active_variants: i64,
    pub open_orders: i64,
    pub pending_productions: i64,
}

#[derive(Default)]
pub struct AnalyticsQuery;

#[Object]
impl AnalyticsQuery {
    async fn operational_overview(&self, ctx: &Context<'_>) -> async_graphql::Result<OperationalOverview> {
        let mut database = ctx.data::<Database>()?.clone();
        let row: (i64, i64, i64, i64) = sqlx::query_as(
            r#"
            select
                (select count(*) from product where status = 'active'),
                (select count(*) from variant where status = 'active'),
                (select count(*) from customer_order where state not in ('completed', 'cancelled', 'rejected')),
                (select count(*) from production_task where state in ('to_plan', 'to_start'))
            "#,
        )
        .fetch_one(database.executor())
        .await?;

        Ok(OperationalOverview {
            active_products: row.0,
            active_variants: row.1,
            open_orders: row.2,
            pending_productions: row.3,
        })
    }
}
