use utoipa::OpenApi;

use crate::{
    responses::{ErrorEnvelope, health::HealthResponse},
    routes::helpers::HealthSuccessEnvelope,
};

#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::helpers::get_health),
    components(schemas(HealthResponse, HealthSuccessEnvelope, ErrorEnvelope))
)]
pub struct ApiDoc;
