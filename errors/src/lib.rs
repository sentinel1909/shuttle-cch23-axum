// errors/src/lib.rs

// dependencies
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// struct to wrap a std::io::Error
pub struct FileError {
    pub inner: std::io::Error,
}

// implement the From trait to convert a std::io::Error into our custom FileError type
impl From<std::io::Error> for FileError {
    fn from(e: std::io::Error) -> Self {
        FileError { inner: e }
    }
}

// implement the IntoResponse trait for our FileError type
impl IntoResponse for FileError {
    fn into_response(self) -> Response {
        let (status, msg) = (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to read the file...",
        );

        (status, msg).into_response()
    }
}
