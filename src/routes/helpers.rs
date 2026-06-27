use axum::{
    Json,
    http::{StatusCode, Uri},
};

use crate::responses::{ErrorEnvelope, ResponseEnvelope, SuccessEnvelope, health::HealthResponse};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = inline(SuccessEnvelope<HealthResponse>)),
        (status = 500, description = "Service unhealthy", body = inline(ErrorEnvelope)),
    )
)]
pub async fn get_health() -> Json<ResponseEnvelope<HealthResponse>> {
    Json(ResponseEnvelope::ok(HealthResponse { healthy: true }))
}

pub async fn fallback_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Path {uri} not found!"))
}
