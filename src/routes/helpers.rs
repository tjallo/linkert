use axum::http::{StatusCode, Uri};

pub async fn get_health() -> (StatusCode, String) {
    (StatusCode::OK, String::from("{\"healthy\": true}"))
}

pub async fn fallback_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Path {uri} not found!"))
}
