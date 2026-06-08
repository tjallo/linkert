use axum::{
    Json,
    http::{StatusCode, Uri},
};

use crate::json_objects::MetricsPayload;

pub async fn fallback_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Path {uri} not found!"))
}

pub async fn get_health() -> (StatusCode, String) {
    (StatusCode::OK, String::from("{\"healthy\": true}"))
}

pub async fn post_metrics(Json(payload): Json<MetricsPayload>) -> (StatusCode, String) {
    println!("Received: {:?}", payload);

    (StatusCode::OK, String::from(format!("{:?}", payload)))
}
