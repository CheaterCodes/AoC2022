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
        .filter(|(a, b)| contains(*a, *b) || contains(*b, *a))
        .count();
        
    let number_overlaps = ranges.iter()
        .filter(|(a, b)| overlaps(*a, *b))
        .count();

    println!("Fully contains: {number_fully_contained}");
    println!("Overlaps: {number_overlaps}");

    Ok(())
}

fn contains(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn overlaps(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}
