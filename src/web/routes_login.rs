use crate::{Error, Result};
use serde::Deserialize;
use axum::{Json, Router};
use serde_json::{json, Value};


pub fn routes() -> Router {
    Router::new()
        .route("/api/login", axum::routing::post(api_login))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username == "admin" && payload.password == "admin" {
        let body = json!({
            "result": {
                "success": true,
            }
        });
        return Ok(Json(body));
    }
    Err(Error::LoginFail)
}