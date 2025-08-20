#![allow(clippy::pedantic)]

use acebau_database::Database;
use actix_web::web::ServiceConfig;

pub mod routes;

#[derive(Debug)]
pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

pub fn registrer_routes(config: &mut ServiceConfig) {
    config.service(routes::products::scope());
}
