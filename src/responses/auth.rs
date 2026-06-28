use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UserRegisterResponse {
    pub username: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserLoginResponse {
    pub jwt: String,
    pub refresh_token: String,
}
