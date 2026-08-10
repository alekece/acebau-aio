#![allow(clippy::pedantic)]

use acebau_database::Database;
use axum::Router;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(|| async { "ok" }))
        .with_state(state)
}
