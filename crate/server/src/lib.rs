#![allow(clippy::pedantic)]

use acebau_database::{models::PrintingEnvironment, Database};
use actix_web::web::ServiceConfig;

pub mod routes;
pub mod envelope;
pub mod query;

pub use envelope::Envelope;
pub use query::ViewQuery;

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
