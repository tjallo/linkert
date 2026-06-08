mod json_objects;
mod route_handlers;

use axum::{
    Router,
    routing::{get, post},
};

use crate::route_handlers::{fallback_route, get_health, post_metrics};

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_router() -> Router<()> {
    Router::<()>::new()
        .route("/health", get(get_health))
        .route("/metrics", post(post_metrics))
        .fallback(fallback_route)
}
