use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Listening on {}", addr);

    let app = mini_server::create_router();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
