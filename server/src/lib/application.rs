// src/lib/application.rs

// dependencies
use axum::{routing::get, Router};
use minus1_endpoints::{error, root};

// function to spin up the axum server and routes
pub fn start_axum_service() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(root))
        .route("/-1/error", get(error));

    Ok(router.into())
}
