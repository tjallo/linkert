use dotenvy::dotenv;

pub struct ConfigVars {
    jwt_secret: String,
    postgres_user: String,
    postgres_password: String,
    postgres_db: String,
    postgres_port: String,
    database_url: String,
    redis_port: String,
    redis_url: String,
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
