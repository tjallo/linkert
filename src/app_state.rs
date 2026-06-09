use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub config_vars: ConfigVars,
    pub redis_connection: Arc<Mutex<redis::Connection>>,
    pub postgres_connection: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Clone, Debug)]
pub struct ConfigVars {
    pub jwt_secret: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
    pub postgres_port: String,
    pub database_url: String,
    pub redis_port: String,
    pub redis_url: String,
}
