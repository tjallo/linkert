use core::panic;

use axum::{
    Json, Router,
    http::{StatusCode, Uri},
    routing::{get, post},
};
use redis::TypedCommands;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    println!("Starting program...");
    let mut redis_con = setup_redis();

    const KEY: &str = "test_key";
    const VAL: &str = "test_val";

    redis_con.set(KEY, VAL).unwrap();

    let out_opt = match redis_con.get(KEY) {
        Err(err) => panic!("{err:?}"),
        Ok(val) => val,
    };

    let out = match out_opt {
        None => String::from(""),
        Some(val) => val,
    };

    println!("Out: {out:?}");
    assert_eq!(String::from(VAL), out);

    start_webserver().await
}

fn setup_redis() -> redis::Connection {
    let client_result = redis::Client::open("redis://127.0.0.1/");

    let connection_result = match client_result {
        Ok(client) => client.get_connection(),
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    };

    match connection_result {
        Ok(client) => client,
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    }
}

async fn start_webserver() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_router() -> Router<()> {
    Router::<()>::new()
        .route("/health", get(get_health))
        .route("/metrics", post(post_metrics))
        .fallback(fallback_route)
}

async fn fallback_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Path {uri} not found!"))
}

async fn get_health() -> (StatusCode, String) {
    (StatusCode::OK, String::from("{\"healthy\": true}"))
}

async fn post_metrics(Json(payload): Json<MetricsPayload>) -> (StatusCode, String) {
    println!("Received: {:?}", payload);

    (StatusCode::OK, String::from(format!("{:?}", payload)))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsPayload {
    hostname: String,
    timestamp: u64,
    cpu_percent: f32,
    mem_used_bytes: u64,
    mem_total_bytes: u64,
    disk_used_bytes: u64,
    disk_total_bytes: u64,
    net_rx_bytes: u64,
    net_tx_bytes: u64,
}
