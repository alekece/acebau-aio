use acebau_database::{Changeset, Database, FetchOptions, Repository, Status};
use actix_web::{
    delete,
    dev::HttpServiceFactory,
    get, patch, post,
    web::{self, Data, Json, Path, Query},
    HttpResponse, Responder, Result,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, Envelope, ViewQuery};

#[derive(Debug, Clone, Copy, Deserialize)]
struct InsertRequest<T> {
    status: Status,
    data: T,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct UpdateRequest<T> {
    status: Status,
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
async fn fetch_all<T>(query: Query<ViewQuery>, context: Data<AppState>) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let view = query.view.unwrap_or_default();

    Ok(Json(
        <Database as Repository<T>>::fetch_all(&mut context.database.clone(), FetchOptions::default())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
            .into_iter()
            .map(|record| Envelope::with_view(record, view))
            .collect::<Vec<_>>(),
    ))
}

#[get("/{id}")]
async fn fetch<T>(id: Path<Uuid>, query: Query<ViewQuery>, context: Data<AppState>) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    <Database as Repository<T>>::fetch_by_id(&mut context.database.clone(), id.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(|record| Json(Envelope::with_view(record, query.view.unwrap_or_default())))
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
    changeset: Json<Changeset<<Database as Repository<T>>::Patch>>,
    id: Path<Uuid>,
    query: Query<ViewQuery>,
    context: Data<AppState>,
) -> Result<impl Responder>
where
    T: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    <Database as Repository<T>>::update(&mut context.database.clone(), id.into_inner(), &changeset.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))
        .map(|record| Json(Envelope::with_view(record, query.view.unwrap_or_default())))
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
        .map(|record| Json(Envelope::new(record)))
}
