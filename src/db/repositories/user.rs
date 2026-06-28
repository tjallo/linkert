use crate::{
    app_state::AppState,
    auth::password::{hash_password, verify_password_hash},
    models::user::User,
    routes::auth::{CreateUserRequest, LoginUserRequest},
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

async fn get_user(state: AppState, username: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_one(&state.postgres_connection)
        .await
}

const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$WZL8yyemsTlsd/Yi7uxaww$kHxBnE5XxhGn1Dr00328BBsIEWH+jGIvsCNaFoO5i1A";

pub async fn validate_user_login(state: AppState, user_request: &LoginUserRequest) -> Option<User> {
    match get_user(state, user_request.username.clone()).await {
        Ok(user) => {
            verify_password_hash(&user.password, &user.password);
            Some(user)
        }
        Err(_) => {
            verify_password_hash("", DUMMY_HASH);
            None
        }
    }
}
