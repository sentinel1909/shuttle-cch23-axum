// day5/src/lib.rs

// dependencies
use axum::{
    extract::{Json, Query},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use domain::Slice;

// "/5" endpoint handler
#[debug_handler]
pub async fn slice_the_loop(slice: Query<Slice>, names: Json<Vec<String>>) -> impl IntoResponse {
    let offset = slice.offset;
    let limit = slice.limit;

    let response = Json(&names[offset..offset + limit]);

    response.into_response()
}
