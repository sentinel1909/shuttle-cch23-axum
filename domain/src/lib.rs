// domain/src/lib.rs

// dependencies
use axum::{
    body::Body,
    extract::FromRef,
    http::header::{CONTENT_LENGTH, CONTENT_TYPE},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

// struct to represent the application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub persist: PersistInstance,
    pub pool: PgPool,
}

// struct to represent the sled data, per the day 4, part 1 challenge
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct StrengthData {
    pub name: String,
    pub strength: i32,
}

// struct to represent the contest data, per the day 4, part 1 challenge
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ContestData {
    pub name: String,
    pub strength: i32,
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: i32,
}

// struct to represent the contest result, per the day 4, part 2 challenge
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ContestResult {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Slice {
    pub offset: usize,
    pub limit: usize,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ElfResponse {
    pub elf: usize,
    #[serde(rename = "elf on a shelf")]
    pub elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    pub shelf_with_no_elf_on_it: usize,
}

// implement the FromRef trait for a PersistInstance
impl FromRef<AppState> for PersistInstance {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.persist.clone()
    }
}

// implement the FromRef trait for a PgPool
impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.pool.clone()
    }
}

// a struct to wrap a vector of bytes (represents the decorations.png image)
#[derive(Clone, Debug, Default)]
pub struct Image(pub Vec<u8>);

impl IntoResponse for Image {
    fn into_response(self) -> Response {
        let content_length = self.0.len();
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, HeaderValue::from_static("image/png"))
            .header(CONTENT_LENGTH, content_length.to_string())
            .body(Body::from(self.0))
            .unwrap()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeData {
    pub stamp: u64,
}

// implement the IntoResponse trait for TimeData
impl IntoResponse for TimeData {
    fn into_response(self) -> Response {
        Response::new(Body::from(self.stamp.to_string()))
    }
}
