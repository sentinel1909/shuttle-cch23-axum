// server/src/bin/httpd.rs

// dependencies
use shuttle_cch23_axum::application::start_axum_service;

// main shuttle runtime
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    start_axum_service()
}
