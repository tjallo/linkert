use utoipa::OpenApi;

use crate::responses::{ErrorEnvelope, health::HealthResponse};

#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::helpers::get_health),
    components(schemas(HealthResponse, ErrorEnvelope))
)]
pub struct ApiDoc;
