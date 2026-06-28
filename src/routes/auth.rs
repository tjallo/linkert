use axum::{Json, extract, http::StatusCode};
use axum_extra::TypedHeader;
use garde::Validate;
use headers::UserAgent;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    app_state::AppState,
    auth::jwt::jwt_encode,
    db::repositories::{
        refresh_token::create_refresh_token,
        user::{create_user, validate_user_login},
    },
    responses::{
        ErrorEnvelope, ResponseEnvelope, SuccessEnvelope,
        auth::{UserLoginResponse, UserRegisterResponse},
    },
};

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateUserRequest {
    #[garde(pattern("^[a-zA-Z0-9_-]+$"), length(min = 3))]
    pub username: String,
    #[garde(ascii, length(min = 15, max = 128))]
    pub password: String,
}

const USER_SUCCESSFULLY_CREATED: &str = "Successfully created User";
const USERNAME_IN_USE: &str = "Username already in use, try again";
const UNPROCESSABLE_ENTITY: &str = "Invalid body given.";
const SERVER_ERROR: &str = "Internal server error. Please try again.";

#[utoipa::path(
    post,
    path = "/auth/register",
    responses(
        (status = 200, description = USER_SUCCESSFULLY_CREATED, body = inline(SuccessEnvelope<UserRegisterResponse>)),
        (status = 409, description = USERNAME_IN_USE, body = inline(ErrorEnvelope)),
        (status = 422, description = UNPROCESSABLE_ENTITY, body = inline(ErrorEnvelope)),
        (status = 500, description = SERVER_ERROR, body = inline(ErrorEnvelope)),
    )
)]
pub async fn register(
    extract::State(state): extract::State<AppState>,
    extract::Json(user): extract::Json<CreateUserRequest>,
) -> (StatusCode, Json<ResponseEnvelope<UserRegisterResponse>>) {
    if let Err(e) = user.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ResponseEnvelope::err(format!(
                "{}: {}",
                StatusCode::UNPROCESSABLE_ENTITY,
                e
            ))),
        );
    }

    match create_user(state, user).await {
        Ok(user) => (
            StatusCode::OK,
            Json(ResponseEnvelope::ok(UserRegisterResponse {
                username: user.username,
            })),
        ),
        Err(err) => {
            let is_conflict = err
                .as_database_error()
                .and_then(|e| e.code())
                .map(|code| code == "23505")
                .unwrap_or(false);

            if is_conflict {
                return (
                    StatusCode::CONFLICT,
                    Json(ResponseEnvelope::err(USERNAME_IN_USE.to_string())),
                );
            }

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseEnvelope::err(SERVER_ERROR.to_string())),
            )
        }
    }
}

#[derive(Deserialize, ToSchema, Validate, Debug)]
pub struct LoginUserRequest {
    #[garde(pattern("^[a-zA-Z0-9_-]+$"))]
    pub username: String,
    #[garde(ascii)]
    pub password: String,
    #[garde(length(max = 32))]
    pub device_name: Option<String>,
}

#[axum::debug_handler]
pub async fn login(
    extract::State(state): extract::State<AppState>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    extract::Json(user_login_req): extract::Json<LoginUserRequest>,
) -> (StatusCode, Json<ResponseEnvelope<UserLoginResponse>>) {
    if let Err(e) = user_login_req.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ResponseEnvelope::err(format!(
                "{}: {}",
                StatusCode::UNPROCESSABLE_ENTITY,
                e
            ))),
        );
    }

    let Some(user) = validate_user_login(state.clone(), &user_login_req).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ResponseEnvelope::err(String::from(
                "You are not authorized to login.",
            ))),
        );
    };

    let Some(jwt) = jwt_encode(state.clone(), &user) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ResponseEnvelope::err(String::from(
                "You are not authorized to login.",
            ))),
        );
    };

    let Ok(refresh_token) = create_refresh_token(
        &state,
        &user,
        Some(&user_agent.as_str()),
        user_login_req.device_name.as_deref(),
    )
    .await
    else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ResponseEnvelope::err(String::from(
                "You are not authorized to login.",
            ))),
        );
    };

    (
        StatusCode::OK,
        Json(ResponseEnvelope::ok(UserLoginResponse {
            jwt,
            refresh_token: refresh_token.refresh_token,
        })),
    )
}
