// day13/src/lib.rs

// dependencies
use axum::extract::State;
use domain::AppState;

// "/13/sql" endpoint handler
pub async fn select_20231213(State(_app_state): State<AppState>) -> Result<String, String> {
    Ok(20231213.to_string())
}
