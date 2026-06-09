mod auth;
mod helpers;
mod urls;

use axum::{Router, routing::get};

use crate::routes::helpers::{fallback_route, get_health};

pub fn create_router() -> Router<()> {
    Router::<()>::new()
        .route("/health", get(get_health))
        .fallback(fallback_route)
}
