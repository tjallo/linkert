use rand::{Rng, distributions::Alphanumeric};

use crate::{
    app_state::AppState,
    models::{refresh_token::RefreshToken, user::User},
};

fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(128)
        .map(|c| (c as char).to_ascii_lowercase())
        .collect()
}

pub async fn create_refresh_token(
    state: &AppState,
    user: &User,
    user_agent: Option<&str>,
    device_name: Option<&str>,
) -> Result<RefreshToken, sqlx::Error> {
    let token = generate_token();
    insert_refresh_token(state, user.id, &token, user_agent, device_name).await
}

async fn insert_refresh_token(
    state: &AppState,
    user_id: i64,
    token: &str,
    user_agent: Option<&str>,
    device_name: Option<&str>,
) -> Result<RefreshToken, sqlx::Error> {
    sqlx::query_as!(
        RefreshToken,
        "INSERT INTO refresh_tokens (user_id, refresh_token, user_agent, device_name)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
        user_id,
        token,
        user_agent,
        device_name,
    )
    .fetch_one(&state.postgres_connection)
    .await
}

pub async fn get_refresh_token(state: &AppState, token: &str) -> Result<RefreshToken, sqlx::Error> {
    sqlx::query_as!(
        RefreshToken,
        "SELECT * FROM refresh_tokens
         WHERE refresh_token = $1
           AND revoked_at IS NULL
           AND expires_at > NOW()",
        token,
    )
    .fetch_one(&state.postgres_connection)
    .await
}

pub async fn revoke_refresh_token(
    state: &AppState,
    token: &str,
) -> Result<RefreshToken, sqlx::Error> {
    sqlx::query_as!(
        RefreshToken,
        "UPDATE refresh_tokens
         SET revoked_at = NOW()
         WHERE refresh_token = $1
         RETURNING *",
        token,
    )
    .fetch_one(&state.postgres_connection)
    .await
}

pub async fn revoke_all_user_tokens(
    state: &AppState,
    user_id: i64,
) -> Result<Vec<RefreshToken>, sqlx::Error> {
    sqlx::query_as!(
        RefreshToken,
        "UPDATE refresh_tokens
         SET revoked_at = NOW()
         WHERE user_id = $1
           AND revoked_at IS NULL
         RETURNING *",
        user_id,
    )
    .fetch_all(&state.postgres_connection)
    .await
}
