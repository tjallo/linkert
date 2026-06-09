use dotenvy::dotenv;

#[derive(Debug)]
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

pub fn get_env_vars() -> ConfigVars {
    ConfigVars {
        jwt_secret: std::env::var("JWT_SECRET").unwrap(),
        postgres_user: std::env::var("POSTGRES_USER").unwrap(),
        postgres_password: std::env::var("POSTGRES_PASSWORD").unwrap(),
        postgres_db: std::env::var("POSTGRES_DB").unwrap(),
        postgres_port: std::env::var("POSTGRES_PORT").unwrap(),
        database_url: std::env::var("DATABASE_URL").unwrap(),
        redis_port: std::env::var("REDIS_PORT").unwrap(),
        redis_url: std::env::var("REDIS_URL").unwrap(),
    }
}

pub fn load_dotenv() -> ConfigVars {
    match dotenv() {
        Err(err) => panic!(".env file not found: {err:?}"),
        Ok(_) => (),
    }

    get_env_vars()
}
