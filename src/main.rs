use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to Hisabi Backend"
}

