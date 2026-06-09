use crate::{
    app_state::AppState, auth::password::hash_password, models::user::User,
    routes::auth::CreateUserRequest,
};

pub async fn create_user(state: AppState, user: CreateUserRequest) -> Result<User, sqlx::Error> {
    let hashed_password = hash_password(&user.password);

    sqlx::query_as!(
        User,
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *",
        user.username,
        hashed_password
    )
    .fetch_one(&state.postgres_connection)
    .await
}

pub async fn get_first_user(state: AppState) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users LIMIT 1",)
        .fetch_one(&state.postgres_connection)
        .await
}
