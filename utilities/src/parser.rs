// utilities/src/lib.rs

// dependencies
use nom::bytes::complete::{tag, take};
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::{preceded, terminated};
use nom::IResult;

// parser to look for the word "elf"
fn elf(input: &str) -> IResult<&str, &str> {
    tag("elf")(input)
}

// parser to look for the word "shelf"
fn shelf(input: &str) -> IResult<&str, &str> {
    tag("shelf")(input)
}

// parser to look for the phrase "elf on a"
fn elf_on_a(input: &str) -> IResult<&str, &str> {
    tag("elf on a ")(input)
}

fn no_whitespace_elf_on_a(input: &str) -> IResult<&str, &str> {
    tag("elf on a")(input)
}

// parser to look for the phrase "elf on a shelf"
fn elf_on_a_shelf(input: &str) -> IResult<&str, &str> {
    preceded(elf_on_a, shelf)(input)
}

fn shelf_no_elf(input: &str) -> IResult<&str, &str> {
    terminated(no_whitespace_elf_on_a, shelf)(input)
}

// function to parse the input string for instances of the word "elf"
pub fn parse_elf(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), elf))(input)
}

// function to parse the input string for instances of the phrase "elf on a shelf"
pub fn parse_elf_on_a_shelf(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), elf_on_a_shelf))(input)
}

// function to parse the input string for instances of the word "shelf"
pub fn parse_shelves(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), shelf))(input)
}

pub fn parse_shelves_no_elves(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), shelf_no_elf))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_elf() {
        let input = "elf elf elf on a shelf";
        let elves = parse_elf(input).unwrap();
        let elf_count = elves.1.len();
        assert_eq!(elf_count, 4);
    }

    #[test]
    fn test_parse_elf_on_a_shelf() {
        let input = "elf elf elf on a shelf";
        let elves_on_shelves = parse_elf_on_a_shelf(input).unwrap();
        let elf_on_shelf_count = elves_on_shelves.1.len();
        assert_eq!(elf_on_shelf_count, 1);
    }

    #[test]
    fn test_parse_shelves() {
        let input = "elf elf elf on a shelf";
        let shelves = parse_shelves(input).unwrap();
        let shelf_count = shelves.1.len();
        assert_eq!(shelf_count, 1);
    }

    #[test]
    fn test_parse_shelves_no_elves() {
        let input = "In Belfast I heard an elf on a shelf on a shelf on a ";
        let shelves_no_elves = parse_shelves_no_elves(input).unwrap();
        let shelf_no_elf_count = shelves_no_elves.1.len();
        assert_eq!(shelf_no_elf_count, 0);
    }
}
