use acebau_database::models::{Product, ProductRepository};
use actix_web::{
    dev::HttpServiceFactory,
    get, post,
    web::{self, Data, Json, Path},
    Responder, Result,
};
use uuid::Uuid;

use crate::AppState;

pub fn scope() -> impl HttpServiceFactory + 'static {
    web::scope("/products").service(fetch_product).service(create_product)
}

#[get("/{id}")]
async fn fetch_product(id: Path<Uuid>, context: Data<AppState>) -> Result<impl Responder> {
    context
        .database
        .clone()
        .fetch_product_by_id(id.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(Json)
}

#[post("/")]
async fn create_product(product: Json<Product>, context: Data<AppState>) -> Result<impl Responder> {
    println!("Creating product: {product:?}");

    let response = context.database.clone().insert_product(product.into_inner()).await;

    println!("Response: {response:?}");

    response
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
        .map(Json)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use acebau_database::{Database, Record};
//     use actix_web::{
//         http::header::ContentType,
//         test::{self, TestRequest},
//         App,
//     };
//     use sqlx::PgPool;

//     #[sqlx::test(migrator = "acebau_database::MIGRATOR")]
//     #[ignore = "requires a running PostgreSQL instance"]
//     async fn test_create_product(pool: PgPool) {
//         let database = Database::new(pool);

//         let app = test::init_service(App::new().app_data(Data::new(AppState::new(database))).service(scope())).await;

//         let request = TestRequest::post()
//             .set_json(Product {
//                 code: "P001".to_string(),
//                 description: Some("Test Product".to_string()),
//                 version: 1,
//             })
//             .insert_header(ContentType::json())
//             .uri("/products/")
//             .to_request();

//         let response = test::call_service(&app, request).await;

//         assert!(response.status().is_success());

//         let product: Record<Product> = test::read_body_json(response).await;

//         assert_eq!(product.code, "P001");
//         assert_eq!(product.description.as_deref(), Some("Test Product"));
//         assert_eq!(product.version, 1);
//     }
// }
