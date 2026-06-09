mod app_state;
mod auth;
mod config;
mod db;
mod models;
mod routes;

use std::sync::{Arc, Mutex};

use crate::{
    app_state::AppState,
    config::load_dotenv,
    db::{postgres::connect_postgres, redis::connect_redis},
    routes::create_router,
};

#[tokio::main]
async fn main() {
    println!("Starting program...");

    let config_vars = load_dotenv();
    let redis_connection = connect_redis(&config_vars.redis_url);
    let postgres_connection = connect_postgres(&config_vars.database_url).await;

    start_webserver(AppState {
        config_vars,
        redis_connection: Arc::new(Mutex::new(redis_connection)),
        postgres_connection,
    })
    .await
}

async fn start_webserver(state: AppState) {
    const HOSTNAME: &str = "0.0.0.0";
    const PORT: &str = "3000";
    let conn_string = format!("{HOSTNAME}:{PORT}");

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&conn_string).await.unwrap();
    println!("Server listening on: {conn_string:?}");
    axum::serve(listener, app).await.unwrap();
}
