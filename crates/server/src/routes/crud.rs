use acebau_database::{Database, FetchOptions, Repository, types::Status};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{
    AppState,
    envelope::{Envelope, FieldMask},
};

#[derive(Debug, Clone, Deserialize)]
struct InsertRequest<T> {
    status: Status,
    #[serde(flatten)]
    data: T,
}

#[derive(Debug, Clone, Deserialize)]
struct UpdateRequest<T> {
    status: Option<Status>,
    #[serde(flatten)]
    data: T,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct StatusResponse {
    status: Status,
}

#[derive(Debug)]
struct ServerError {
    status: StatusCode,
    message: String,
}

impl ServerError {
    fn internal(error: impl ToString) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }

    fn not_found(error: impl ToString) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: error.to_string(),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (self.status, Json(serde_json::json!({ "error": self.message }))).into_response()
    }
}

pub fn router<T>() -> Router<AppState>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
    acebau_database::Record<T>: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
    <Database as Repository<T>>::Changeset: DeserializeOwned + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(fetch_all::<T>).post(insert::<T>))
        .route("/{id}", get(fetch::<T>).patch(update::<T>).delete(delete::<T>))
        .route("/{id}/status", get(get_status::<T>))
}

async fn fetch_all<T>(
    State(context): State<AppState>,
    Query(field_mask): Query<FieldMask>,
) -> Result<Response, ServerError>
where
    T: Serialize + Send + Sync,
    acebau_database::Record<T>: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let mut database = context.database.clone();
    let records = database
        .fetch_all(FetchOptions::default())
        .await
        .map_err(ServerError::internal)?;

    Ok(Envelope::ok(records).into_response(&field_mask))
}

async fn fetch<T>(
    State(context): State<AppState>,
    Path(id): Path<Uuid>,
    Query(field_mask): Query<FieldMask>,
) -> Result<Response, ServerError>
where
    T: Serialize + Send + Sync,
    acebau_database::Record<T>: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let mut database = context.database.clone();
    let record = database.fetch_by_id(id).await.map_err(ServerError::not_found)?;

    Ok(Envelope::ok(record).into_response(&field_mask))
}

async fn delete<T>(State(context): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode, ServerError>
where
    T: Serialize + Send + Sync,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let mut database = context.database.clone();
    database.delete(id).await.map_err(ServerError::not_found)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_status<T>(
    State(context): State<AppState>,
    Path(id): Path<Uuid>,
    Query(field_mask): Query<FieldMask>,
) -> Result<Response, ServerError>
where
    T: Serialize + Send + Sync,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let mut database = context.database.clone();
    let status = database.status(id).await.map_err(ServerError::not_found)?;

    Ok(Envelope::ok(StatusResponse { status }).into_response(&field_mask))
}

async fn update<T>(
    State(context): State<AppState>,
    Path(id): Path<Uuid>,
    Query(field_mask): Query<FieldMask>,
    Json(changeset): Json<UpdateRequest<<Database as Repository<T>>::Changeset>>,
) -> Result<Response, ServerError>
where
    T: Serialize + Send + Sync,
    acebau_database::Record<T>: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
    <Database as Repository<T>>::Changeset: DeserializeOwned + Send + Sync,
{
    let UpdateRequest { data, status } = changeset;
    let mut database = context.database.clone();
    let record = database
        .update(id, &data, status)
        .await
        .map_err(ServerError::not_found)?;

    Ok(Envelope::ok(record).into_response(&field_mask))
}

async fn insert<T>(
    State(context): State<AppState>,
    Query(field_mask): Query<FieldMask>,
    Json(data): Json<InsertRequest<T>>,
) -> Result<Response, ServerError>
where
    T: Serialize + DeserializeOwned + Send + Sync,
    acebau_database::Record<T>: Serialize,
    Database: Repository<T>,
    <Database as Repository<T>>::Error: ToString,
{
    let InsertRequest { data, status } = data;
    let mut database = context.database.clone();
    let record = database.insert(&data, status).await.map_err(ServerError::internal)?;

    Ok(Envelope::created(record).into_response(&field_mask))
}
