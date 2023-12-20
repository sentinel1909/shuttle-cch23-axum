// minus1/src/lib.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse};

// index route
pub async fn root() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}
