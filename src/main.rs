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
    (StatusCode::NOT_FOUND, format!("Route {uri} not found!"))
}

fn create_router() -> Router<()> {
    const HEALTHY_RESPONSE: &str = "{\"healthy\": true}";

    Router::<()>::new()
        .route("/health", get(|| async { HEALTHY_RESPONSE }))
        .fallback(fallback_route)
}
