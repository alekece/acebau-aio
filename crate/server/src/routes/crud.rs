use acebau_database::{Database, FetchOptions, Repository, types::Status};
use actix_web::{
    HttpResponse, Responder, Result, delete,
    dev::HttpServiceFactory,
    get, patch, post,
    web::{self, Data, Json, Path, Query},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, Envelope};

#[derive(Debug, Clone, Copy, Deserialize)]
struct InsertRequest<T> {
    status: Status,
    #[serde(flatten)]
    data: T,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct UpdateRequest<T> {
    status: Option<Status>,
    #[serde(flatten)]
    data: T,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct StatusResponse {
    status: Status,
}

pub fn scope<T>(name: &str) -> impl HttpServiceFactory + 'static {
    web::scope(name)
        .service(fetch_all)
        .service(fetch)
        .service(insert)
        .service(update)
        .service(get_status)
}

#[get("")]
async fn fetch_all<T>(context: Data<AppState>) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    Ok(Json(
        <Database as Repository<T>>::fetch_all(&mut context.database.clone(), FetchOptions::default())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
            .into_iter()
            .map(Envelope)
            .collect::<Vec<_>>(),
    ))
}

#[get("/{id}")]
async fn fetch<T>(id: Path<Uuid>, context: Data<AppState>) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    <Database as Repository<T>>::fetch_by_id(&mut context.database.clone(), id.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(Envelope)
}

#[delete("/{id}")]
async fn delete<T>(id: Path<Uuid>, context: Data<AppState>) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    <Database as Repository<T>>::delete(&mut context.database.clone(), id.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(|_| HttpResponse::NoContent())
}

#[get("/{id}/status")]
async fn get_status<T>(id: Path<Uuid>, context: Data<AppState>) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    <Database as Repository<T>>::status(&mut context.database.clone(), id.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(|status| Json(StatusResponse { status }))
}

#[patch("/{id}")]
async fn update<T>(
    changeset: Json<UpdateRequest<<Database as Repository<T>>::Changeset>>,
    id: Path<Uuid>,
    context: Data<AppState>,
) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let UpdateRequest { data, status } = changeset.into_inner();

    <Database as Repository<T>>::update(&mut context.database.clone(), id.into_inner(), &data, status)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(Envelope)
}

#[post("")]
async fn insert<T>(data: Json<InsertRequest<T>>, context: Data<AppState>) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let InsertRequest { data, status } = data.into_inner();

    <Database as Repository<T>>::insert(&mut context.database.clone(), &data, status)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
        .map(Envelope)
}
