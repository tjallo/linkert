use utoipa::OpenApi;

use crate::responses::{ErrorEnvelope, auth::UserRegisterResponse, health::HealthResponse};

#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::helpers::get_health, crate::routes::auth::register),
    components(schemas(HealthResponse, UserRegisterResponse, ErrorEnvelope))
)]
pub struct ApiDoc;
