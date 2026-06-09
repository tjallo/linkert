use axum::{
    extract,
    http::{StatusCode, Uri},
};

use crate::{app_state::AppState, db::repositories::user::get_first_user};

pub async fn get_health() -> (StatusCode, String) {
    (StatusCode::OK, String::from("{\"healthy\": true}"))
}

pub async fn fallback_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Path {uri} not found!"))
}

// TODO: Remove
pub async fn get_first_user_route(
    extract::State(state): extract::State<AppState>,
) -> (StatusCode, String) {
    let user = get_first_user(state).await.unwrap();

    (StatusCode::OK, format!("{user:?}"))
}
