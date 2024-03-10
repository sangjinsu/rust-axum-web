use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response{
        match self {
            Error::LoginFail => {
                (StatusCode::BAD_REQUEST, "Login Fail".to_string()).into_response()
            }
        }
    }
}