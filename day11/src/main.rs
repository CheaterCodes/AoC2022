use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day11/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap);

    Ok(())
}
