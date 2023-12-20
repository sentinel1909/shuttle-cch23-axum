// src/lib/application.rs

// dependencies
use axum::{
    routing::{get, post},
    Router,
};
use day12_endpoints::{timekeeper_get, timekeeper_post};
use domain::AppState;
use minus1_endpoints::{error, root};
use shuttle_persist::PersistInstance;

// function to spin up the axum server and routes
pub async fn start_axum_service(persist: PersistInstance) -> shuttle_axum::ShuttleAxum {
    let state = AppState { persist };
    let router = Router::new()
        .route("/", get(root))
        .route("/-1/error", get(error))
        .route("/12/save/:packet_id", post(timekeeper_post))
        .route("/12/load/:packet_id", get(timekeeper_get))
        .with_state(state);

    Ok(router.into())
}
