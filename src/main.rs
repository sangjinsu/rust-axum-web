#![allow(unused)]

pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{get, get_service};
use serde::Deserialize;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_fallback());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


fn routes_fallback() -> Router {
    async fn handler_fallback() -> impl IntoResponse {
        Html("Hello, axum!".to_string())
    }
    Router::new().route(
        "/",
        get(handler_fallback),
    )
}


fn routes_hello() -> Router {
    Router::new()
        .route(
            "/hello",
            get(handler_hello),
        ).route(
        "/hello2/:name",
        get(handler_hello2),
    )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("params: {:?}", params);

    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello, {}!", name))
}


async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, {}!", name))
}