use utoipa::OpenApi;

use crate::responses::{
    ErrorEnvelope,
    auth::{UserLoginResponse, UserRegisterResponse},
    health::HealthResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::helpers::get_health,
        crate::routes::auth::register,
        crate::routes::auth::login
    ),
    components(schemas(HealthResponse, UserRegisterResponse, UserLoginResponse, ErrorEnvelope))
)]
pub struct ApiDoc;
