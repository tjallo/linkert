pub mod auth;
pub mod helpers;
mod urls;

use axum::{
    Json, Router,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    AppState,
    openapi::ApiDoc,
    routes::{
        auth::{login, register},
        helpers::{fallback_route, get_health},
    },
};

pub fn create_router(state: AppState) -> Router<()> {
    let stateful = Router::new()
        .route("/health", get(get_health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .fallback(fallback_route)
        .with_state(state);

    Router::new()
        .merge(stateful)
        .route("/openapi.json", get(|| async { Json(ApiDoc::openapi()) }))
}
