use axum::{
    Json,
    http::{StatusCode, Uri},
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::responses::{ErrorEnvelope, ResponseEnvelope, health::HealthResponse};

#[derive(Serialize, ToSchema)]
pub struct HealthSuccessEnvelope {
    pub success: bool,
    pub data: HealthResponse,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthSuccessEnvelope),
        (status = 500, description = "Service unhealthy", body = ErrorEnvelope),
    )
)]
pub async fn get_health() -> Json<ResponseEnvelope<HealthResponse>> {
    Json(ResponseEnvelope::ok(HealthResponse { healthy: true }))
}

pub async fn fallback_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Path {uri} not found!"))
}
