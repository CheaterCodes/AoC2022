use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day4/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();


    let ranges = lines.iter()
        .map(|line| line
            .splitn(4, ['-', ','])
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
        )
        .map(|nums| ((nums[0], nums[1]), (nums[2], nums[3])))
        .collect::<Vec<_>>();

    let number_fully_contained = ranges.iter()
        .filter(|((min_a, max_a), (min_b, max_b))| min_a >= min_b && max_a <= max_b || min_b >= min_a && max_b <= max_a)
        .count();

    println!("Fully contains: {number_fully_contained}");

    Ok(())
}
