mod auth;
mod config;
mod db;
mod models;
mod routes;
mod server;

use crate::{config::load_dotenv, db::redis::connect_redis, server::start_webserver};

#[tokio::main]
async fn main() {
    println!("Starting program...");

    let config = load_dotenv();
    let mut _redis_con = connect_redis(&config.redis_url);

    start_webserver().await
}
