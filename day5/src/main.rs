use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day4/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

    Ok(())
}
