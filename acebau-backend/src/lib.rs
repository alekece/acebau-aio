#![allow(clippy::pedantic)]

use actix_web::web::ServiceConfig;
use std::fmt;

pub mod database;
pub mod routes;

struct AppState {}

pub fn configure_app(config: &mut ServiceConfig) {
    config
        .app_data(AppState {})
        .service(routes::materials::fetch)
        .service(routes::materials::fetch_all);
}
