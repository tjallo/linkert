use axum::{Json, extract, http::StatusCode};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    app_state::AppState,
    db::repositories::user::create_user,
    responses::{ErrorEnvelope, ResponseEnvelope, SuccessEnvelope, auth::UserRegisterResponse},
};

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    responses(
        (status = 200, description = "Successfully created User", body = inline(SuccessEnvelope<UserRegisterResponse>)),
        (status = 409, description = "Username already in use, try again", body = inline(ErrorEnvelope)),
    )
)]
pub async fn register(
    extract::State(state): extract::State<AppState>,
    extract::Json(user): extract::Json<CreateUserRequest>,
) -> (StatusCode, Json<ResponseEnvelope<UserRegisterResponse>>) {
    match create_user(state, user).await {
        Ok(user) => (
            StatusCode::OK,
            Json(ResponseEnvelope::ok(UserRegisterResponse {
                username: user.username,
            })),
        ),
        Err(_) => (
            StatusCode::CONFLICT,
            Json(ResponseEnvelope::err(String::from(
                "Failed to create user with that username, please try again",
            ))),
        ),
    }
}
