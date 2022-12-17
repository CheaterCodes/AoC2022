use std::error::Error;

use regex::Regex;

const REGEX: &'static str = "";

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day17/input.txt")?;
    let regex = Regex::new(REGEX)?;
    let captures = regex.captures(&input).unwrap();

    Ok(())
}
