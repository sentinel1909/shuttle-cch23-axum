// domain/src/lib.rs

// dependencies
use axum::{body::Body, response::IntoResponse, response::Response};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

// struct to represent the application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub persist: PersistInstance,
    pub pool: PgPool,
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
