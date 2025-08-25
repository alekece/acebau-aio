use acebau_database::{FetchOptions, Repository, models::PrintingEnvironment, types::Status};
use actix_web::{
    Responder, Result,
    dev::HttpServiceFactory,
    get, post,
    web::{self, Data, Json, Path, Query},
};
use uuid::Uuid;

use crate::{AppState, Envelope, ViewQuery};

pub fn scope() -> impl HttpServiceFactory + 'static {
    web::scope("/printing_environments")
        .service(fetch_all_printing_environments)
        .service(fetch_printing_environment)
        .service(create_printing_environment)
}

#[get("/")]
async fn fetch_all_printing_environments(context: Data<AppState>, query: Query<ViewQuery>) -> Result<impl Responder> {
    let view = query.view.unwrap_or_default();

    Ok(Json(
        context
            .database
            .clone()
            .fetch_all(FetchOptions::default())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
            .into_iter()
            .map(|record| Envelope::with_view(record, view))
            .collect::<Vec<_>>(),
    ))
}

#[get("/{id}")]
async fn fetch_printing_environment(id: Path<Uuid>, context: Data<AppState>) -> Result<impl Responder> {
    context
        .database
        .clone()
        .fetch_by_id(id.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(Json)
}

#[post("/")]
async fn create_printing_environment(
    printing_environment: Json<PrintingEnvironment>,
    context: Data<AppState>,
) -> Result<impl Responder> {
    println!("Creating printing_environment: {printing_environment:?}");

    let response = context
        .database
        .clone()
        .insert(&printing_environment.into_inner(), Status::Active)
        .await;

    println!("Response: {response:?}");

    response
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
        .map(Json)
}
