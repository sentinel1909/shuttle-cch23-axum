// day7/src/lib.rs

// dependencies
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use axum_extra::{headers::Cookie, TypedHeader};
use axum_macros::debug_handler;
use base64::engine::general_purpose;
use base64::Engine;

// '/7/decode" endpoint handler
#[debug_handler]
pub async fn decode_the_receipe(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {
    match cookie.get("recipe") {
        Some(value) => {
            let cookie_value = value.split('=').collect::<Vec<&str>>()[0];
            let decoded_cookie = match general_purpose::STANDARD_NO_PAD.decode(cookie_value) {
                Ok(value) => value,
                Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            };
            let decoded_recipe = match String::from_utf8(decoded_cookie) {
                Ok(recipe) => recipe,
                Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            };
            let response = Json(decoded_recipe);
            response.into_response()
        }
        None => (StatusCode::BAD_REQUEST, "No cookie").into_response(),
    }
}
