// day4/src/lib.rs

// dependencies
use axum::{extract::Json, response::IntoResponse};
use domain::{ContestData, ContestResult, StrengthData};

// "/4/strength" endpoint, calculate the strength of all the reindeer
pub async fn calculate_total_strength(
    Json(strength_data): Json<Vec<StrengthData>>,
) -> impl IntoResponse {
    let mut total_strength = 0;
    for entry in strength_data {
        total_strength += entry.strength;
    }

    total_strength.to_string().into_response()
}

// "/4/contest" endpoint, function to determine the outcome of the reindeer comparison contest
pub async fn get_contest_results(Json(contest_data): Json<Vec<ContestData>>) -> impl IntoResponse {
    let fastest = contest_data
        .iter()
        .max_by(|a, b| a.speed.total_cmp(&b.speed))
        .unwrap();
    let tallest = contest_data.iter().max_by_key(|a| a.height).unwrap();
    let magician = contest_data
        .iter()
        .max_by_key(|a| a.snow_magic_power)
        .unwrap();
    let consumer = contest_data
        .iter()
        .max_by_key(|a| a.candies_eaten_yesterday)
        .unwrap();

    // create the JSON message for the response body
    let response = Json(ContestResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    });

    response.into_response()
}
