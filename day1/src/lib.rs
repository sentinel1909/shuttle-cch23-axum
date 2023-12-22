// day1/src/lib.rs

// dependencies
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;

// "/1/<num1>/<num2>" endpoint handler to calibrate packet ids
#[debug_handler]
pub async fn calibrate_ids(Path(packets): Path<String>) -> impl IntoResponse {
    // split up the incoming path into segments and convert them to integers
    let path_segments: Vec<Result<i32, _>> = packets
        .split('/')
        .map(|segment| segment.parse::<i32>())
        .collect();

    // collect everything thats an integer into a Vector, this represents our packets
    let packets: Vec<i32> = path_segments
        .into_iter()
        .filter_map(|path_segment| path_segment.ok())
        .collect();

    // determine the number of packets that have come in, if only 2, that means
    // we're calibrating packet IDs, if it's anything else, but not more than 21,
    // we're calculating sled IDs
    match packets.len() {
        2 => {
            let bitwise_or = packets[0] ^ packets[1];
            let calibrated_packet_id = bitwise_or.pow(3);
            calibrated_packet_id.to_string().into_response()
        }

        // there's a match guard on this path to catch if there are not more than 20 packets
        packet if packet < 21 => {
            let bitwise_or = packets.iter().fold(0, |packet, &x| packet ^ x);
            let calibrated_sled_id = bitwise_or.pow(3);
            calibrated_sled_id.to_string().into_response()
        }

        // respond with a bad request if neither route is matched
        _ => (StatusCode::BAD_REQUEST, "Too many packets! Enter 20 or less to calibrate the sled id!").into_response(),
    }
}
