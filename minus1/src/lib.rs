// minus1/src/lib.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;

// "/" index route
#[debug_handler]
pub async fn root() -> impl IntoResponse {
    StatusCode::OK
}

// "-1/error" fake error route
#[debug_handler]
pub async fn error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}
