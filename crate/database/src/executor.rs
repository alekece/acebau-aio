use sqlx::Postgres;

pub trait Executor<'a> {
    type Executor: sqlx::Executor<'a, Database = Postgres>;

    fn executor(&'a mut self) -> Self::Executor;
}
