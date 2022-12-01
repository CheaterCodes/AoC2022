use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day1/input.txt")?;
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut current = 0;
    
    for line in reader.lines().map(Result::unwrap) {
        if let Ok(num) = line.parse::<i32>() {
            current += num;
        } else {
            max = max.max(current);
            current = 0;
        }
    }

    println!("Total of max: {max}");

    Ok(())
}
