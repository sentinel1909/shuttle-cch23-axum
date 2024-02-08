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
    let offset = slice.offset.unwrap_or(0);
    let limit = slice.limit.unwrap_or(names.len());
    let split = slice.split; 

    let slice = &names[offset..std::cmp::min(offset + limit, names.len())];

    match split {
        Some(size) => {
            let mut result = Vec::new();
            let mut chunk = Vec::new();
            for name in slice {
                if chunk.len() >= size {
                    result.push(chunk);
                    chunk = Vec::new();
                }
                chunk.push(name.clone());
            }
            if !chunk.is_empty() {
                result.push(chunk);
            }
            let response = Json(result);
            response.into_response()
        },
        None => {
            
            let response = Json(slice);
            response.into_response()
        }   
    }
}
