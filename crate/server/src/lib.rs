#![allow(clippy::pedantic)]

use acebau_database::{Database, models::PrintingEnvironment};
use actix_web::web::ServiceConfig;

pub mod envelope;
pub mod routes;

pub use envelope::Envelope;

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
    config
        .service(routes::products::scope())
        .service(routes::crud::scope::<PrintingEnvironment>("/printing_environments"));
}
