mod auth;
mod helpers;
mod urls;

use axum::{Router, routing::get};

use crate::{
    AppState,
    routes::helpers::{fallback_route, get_health},
};

pub fn create_router(state: AppState) -> Router<()> {
    Router::<AppState>::new()
        .route("/health", get(get_health))
        .fallback(fallback_route)
        .with_state(state)
}
