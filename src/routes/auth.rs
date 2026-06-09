use axum::{extract, http::StatusCode};
use serde::Deserialize;

use crate::{app_state::AppState, db::repositories::user::create_user};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(
    extract::State(state): extract::State<AppState>,
    extract::Json(user): extract::Json<CreateUserRequest>,
) -> (StatusCode, String) {
    create_user(state, user).await;
    (StatusCode::OK, String::from("{\"healthy\": true}"))
}
