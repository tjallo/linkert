use sqlx::{PgPool, Pool, Postgres};

pub async fn connect_postgres(database_url: &str) -> Pool<Postgres> {
    match PgPool::connect(database_url).await {
        Ok(pool) => pool,
        Err(err) => panic!("Failed to connect to Postgres Pool: {err:?}"),
    }
}
