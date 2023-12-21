// day11/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 11 Challenge Endpoints

// dependencies
use axum_macros::debug_handler;
use domain::Image;
use errors::FileError;
use std::fs::File;
use std::io::Read;

// a constant which represents the file path
const FILE: &str = "/home/jeff/dev/source/repos/rust/shuttle-cch23/day11/assets/decoration.png";

// endpoint handler to return a static image of Christmas decorations
#[debug_handler]
pub async fn static_file_get() -> Result<Image, FileError> {
    // read in the image file
    let file = File::open(FILE);

    // create a buffer to store the file
    let mut buf: Image = Image(Vec::new());

    // read the image file into the buffer
    file?.read_to_end(&mut buf.0)?;

    Ok(buf)
}
