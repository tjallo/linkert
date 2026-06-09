use crate::routes::create_router;

pub async fn start_webserver() {
    const HOSTNAME: &str = "0.0.0.0";
    const PORT: &str = "3000";
    let conn_string = format!("{HOSTNAME}:{PORT}");

    let app = create_router();

    let listener = tokio::net::TcpListener::bind(&conn_string).await.unwrap();
    println!("Server listening on: {conn_string:?}");
    axum::serve(listener, app).await.unwrap();
}
