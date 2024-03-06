#![allow(unused)]


use std::net::SocketAddr;
use axum::response::Html;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/hello",
        get(|| async {
            Html("Hello World")
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
