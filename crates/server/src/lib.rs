#![allow(clippy::pedantic)]

use acebau_database::{
    Database,
    models::{Material, MaterialProvider, Part, PrintingEnvironment, Product, ProductVariant, ProductVariantPart},
};
use axum::Router;

pub mod envelope;
pub mod routes;

pub use envelope::Envelope;

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
        .nest("/printing_environments", routes::crud::router::<PrintingEnvironment>())
        .nest("/products", routes::crud::router::<Product>())
        .nest("/product_variants", routes::crud::router::<ProductVariant>())
        .nest("/product_variant_parts", routes::crud::router::<ProductVariantPart>())
        .nest("/parts", routes::crud::router::<Part>())
        .nest("/materials", routes::crud::router::<Material>())
        .nest("/material_providers", routes::crud::router::<MaterialProvider>())
        .with_state(state)
}
