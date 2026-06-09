mod auth;
mod config;
mod db;
mod models;
mod routes;

use dotenvy::dotenv;

use crate::{db::redis::connect_redis, routes::create_router};

#[tokio::main]
async fn main() {
    println!("Starting program...");
    load_dotenv();
    let mut _redis_con = connect_redis();
    start_webserver().await
}

fn load_dotenv() {
    match dotenv() {
        Err(err) => panic!(".env file not found: {err:?}"),
        Ok(_) => (),
    }
}

async fn start_webserver() {
    const HOSTNAME: &str = "0.0.0.0";
    const PORT: &str = "3000";
    let conn_string = format!("{HOSTNAME}:{PORT}");

    let app = create_router();

    let listener = tokio::net::TcpListener::bind(&conn_string).await.unwrap();
    println!("Server listening on: {conn_string:?}");
    axum::serve(listener, app).await.unwrap();
}
