use axum::{
    Router,
    http::{StatusCode, Uri},
    routing::get,
};

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fallback_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Path {uri} not found!"))
}

async fn health_route() -> (StatusCode, String) {
    (StatusCode::OK, "{\"healthy\": true}".to_string())
}

fn create_router() -> Router<()> {
    Router::<()>::new()
        .route("/health", get(health_route))
        .fallback(fallback_route)
}
