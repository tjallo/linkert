pub mod auth;
mod helpers;
mod urls;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    routes::{
        auth::register,
        helpers::{fallback_route, get_health},
    },
};

pub fn create_router(state: AppState) -> Router<()> {
    Router::<AppState>::new()
        .route("/health", get(get_health))
        .route("/auth/register", post(register))
        .fallback(fallback_route)
        .with_state(state)
}
