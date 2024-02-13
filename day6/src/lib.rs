// day6/src/lib.rs

// dependencies
use axum::{extract::Json, response::IntoResponse};
use domain::ElfResponse;
use utilities::{parse_elf, parse_elf_on_a_shelf, parse_shelves_no_elves};

// "/6" endpoint handler
pub async fn count_elves(body: String) -> impl IntoResponse {
    let num_elves = parse_elf(&body).unwrap();
    let num_elves_on_shelves = parse_elf_on_a_shelf(&body).unwrap();
    let num_shelves_with_no_elves = parse_shelves_no_elves(&body).unwrap();
    let elf_count = num_elves.1.len();
    let elf_on_a_shelf_count = num_elves_on_shelves.1.len();
    let shelf_with_no_elf_count = num_shelves_with_no_elves.1.len();

    let response = Json(ElfResponse {
        elf: elf_count,
        elf_on_a_shelf: elf_on_a_shelf_count,
        shelf_with_no_elf_on_it: shelf_with_no_elf_count,
    });

    response.into_response()
}
