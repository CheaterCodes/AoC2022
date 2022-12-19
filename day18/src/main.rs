use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day18/input.txt")?;

    Ok(())
}
