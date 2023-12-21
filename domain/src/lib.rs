// domain/src/lib.rs

// dependencies
use axum::{
    body::Body,
    extract::FromRef,
    http::header::{CONTENT_LENGTH, CONTENT_TYPE},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

// struct to represent the application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub persist: PersistInstance,
    pub pool: PgPool,
}

// implement the FromRef trait for a PersistInstance
impl FromRef<AppState> for PersistInstance {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.persist.clone()
    }
}

// implement the FromRef trait for a PgPool
impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.pool.clone()
    }
}

// a struct to wrap a vector of bytes (represents the decorations.png image)
#[derive(Clone, Debug, Default)]
pub struct Image(pub Vec<u8>);

impl IntoResponse for Image {
    fn into_response(self) -> Response {
        let content_length = self.0.len();
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, HeaderValue::from_static("image/png"))
            .header(CONTENT_LENGTH, content_length.to_string())
            .body(Body::from(self.0))
            .unwrap()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeData {
    pub stamp: u64,
}

// implement the IntoResponse trait for TimeData
impl IntoResponse for TimeData {
    fn into_response(self) -> Response {
        Response::new(Body::from(self.stamp.to_string()))
    }
}
