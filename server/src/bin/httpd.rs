// server/src/bin/httpd.rs

// dependencies
use shuttle_cch23_axum::application::start_axum_service;
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

// main shuttle runtime
#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    start_axum_service(persist, pool).await
}
