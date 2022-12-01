use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day1/input.txt")?;
    let reader = BufReader::new(file);

    let mut totals = Vec::new();
    let mut current = 0;
    
    for line in reader.lines().map(Result::unwrap) {
        if let Ok(num) = line.parse::<i32>() {
            current += num;
        } else {
            totals.push(current);
            current = 0;
        }
    }

    totals.sort();
    totals.reverse();

    println!("Total of max: {}", totals[0]);

    println!("Total of top 3: {}", totals[0] + totals[1] + totals[2]);

    Ok(())
}
