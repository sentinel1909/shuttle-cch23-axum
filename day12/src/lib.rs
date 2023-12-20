// day 12/src/lib.rs

use std::time::UNIX_EPOCH;

// dependencies
use axum::extract::{Path, State};
use axum_macros::debug_handler;
use domain::{AppState, TimeData};
use std::time::{SystemTime, SystemTimeError};

// function to get the time in seconds
fn get_time_in_seconds() -> Result<u64, SystemTimeError> {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH)?;
    let seconds_timestamp = duration_since_epoch.as_secs();

    Ok(seconds_timestamp)
}

// "/12/save/<string>" get endpoint handler
#[debug_handler]
pub async fn timekeeper_get(
    Path(packet_id): Path<String>,
    State(state): State<AppState>,
) -> Result<TimeData, String> {
    let time_loaded = get_time_in_seconds().to_owned().unwrap();

    let result: TimeData = match state.persist.load(&packet_id) {
        Ok(time) => time,
        Err(error) => return Err(format!("Error: {}", error)),
    };

    let time_difference: TimeData = TimeData {
        stamp: time_loaded - result.stamp,
    };

    Ok(time_difference)
}

// "/12/load/<string>" post endpoint handler
#[debug_handler]
pub async fn timekeeper_post(
    Path(packet_id): Path<String>,
    State(state): State<AppState>,
) -> Result<String, String> {
    let time_saved = TimeData {
        stamp: get_time_in_seconds().to_owned().unwrap(),
    };

    let result = match state.persist.save(&packet_id, time_saved.stamp) {
        Ok(_) => "Successfully saved the timestamp assocated with the incoming packet".to_string(),
        Err(error) => return Err(format!("Error: {}", error)),
    };

    Ok(result)
}
