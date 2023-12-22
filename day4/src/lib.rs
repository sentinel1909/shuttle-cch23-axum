// day4/src/lib.rs

// dependencies
use axum::{
    extract::Json,
    response::IntoResponse,
};
use domain::StrengthData;

// "/4/strength" endpoint, calculate the strength of all the reindeer
pub async fn calculate_total_strength(Json(strength_data): Json<Vec<StrengthData>>) -> impl IntoResponse {
    
    let mut total_strength = 0;
    for entry in strength_data {
        total_strength += entry.strength;
    }

    total_strength.to_string().into_response()
}

