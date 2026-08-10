#![allow(clippy::pedantic)]

use acebau_database::{
    models::{Material, MaterialProvider, Part, PrintingEnvironment, Product, ProductVariant, ProductVariantPart},
    Database,
};
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
        .service(routes::crud::scope::<PrintingEnvironment>("/printing_environments"))
        .service(routes::crud::scope::<Product>("/products"))
        .service(routes::crud::scope::<ProductVariant>("/product_variants"))
        .service(routes::crud::scope::<ProductVariantPart>("/product_variants"))
        .service(routes::crud::scope::<Part>("/parts"))
        .service(routes::crud::scope::<Material>("/materials"))
        .service(routes::crud::scope::<MaterialProvider>("/material_providers"));
}
