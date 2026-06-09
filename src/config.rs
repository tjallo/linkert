use dotenvy::dotenv;

use crate::app_state::ConfigVars;

fn get_var(key: &str) -> String {
    match std::env::var(key) {
        Ok(var) => var,
        Err(_) => panic!("Following var not found in .env: {key:?}"),
    }
}

fn get_env_vars() -> ConfigVars {
    ConfigVars {
        jwt_secret: get_var("JWT_SECRET"),
        postgres_user: get_var("POSTGRES_USER"),
        postgres_password: get_var("POSTGRES_PASSWORD"),
        postgres_db: get_var("POSTGRES_DB"),
        postgres_port: get_var("POSTGRES_PORT"),
        database_url: get_var("DATABASE_URL"),
        redis_port: get_var("REDIS_PORT"),
        redis_url: get_var("REDIS_URL"),
    }
}

pub fn load_dotenv() -> ConfigVars {
    match dotenv() {
        Err(err) => panic!(".env file not found: {err:?}"),
        Ok(_) => (),
    }

    get_env_vars()
}
