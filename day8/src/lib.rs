// day8/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 8 Challenge

// dependencies
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Response, IntoResponse},
};
use axum_macros::debug_handler;
use reqwest::{Client, Error};

use domain::PokemonWeight;

// wrapper struct for a reqwest::Error
pub struct ReqwestError {
    pub inner: Error
}

// implement the IntoResponse trait for ReqwestError
impl IntoResponse for ReqwestError {
    fn into_response(self) -> Response {
        let body = "Unable to fetch the weight...".to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

// implement the From trait for ReqwestError, converts a reqwest::Error to a ReqwestError type
impl From<Error> for ReqwestError {
    fn from(error: Error) -> Self {
        ReqwestError {
            inner: error
        }
    }
}

// endpoint which returns the weight in kilograms of a Pokemnon
#[debug_handler]
pub async fn get_weight(Path(pokedex_number): Path<i32>) -> Result<impl IntoResponse, ReqwestError> {
    // create a reqwest client
    let client = Client::new();

    // get the weight in kilograms of the Pokemon
    let pokemon = client
        .get(&format!(
            "https://pokeapi.co/api/v2/pokemon/{}",
            pokedex_number
        ))
        .send()
        .await?
        .json::<PokemonWeight>()
        .await?;

    // create the response body
    let response_msg = (pokemon.weight / 10f32).to_string();

    // return the response
    Ok(response_msg.into_response())
}
