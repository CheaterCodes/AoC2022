use std::{fs::File, error::Error, io::{BufReader, BufRead}};

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for char {
    fn priority(&self) -> i32 {
        if self.is_ascii_lowercase() {
            *self as i32 - 'a' as i32 + 1
        } else if self.is_ascii_uppercase() {
            *self as i32 - 'A' as i32 + 27
        } else {
            panic!()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day3/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap);

    let rucksacks = lines.map(|l| {
        let parts = l.split_at(l.len() / 2);
        (parts.0.chars().collect::<Vec<_>>(), parts.1.chars().collect::<Vec<_>>())
    }).collect::<Vec<_>>();


    let sum: i32 = rucksacks.iter().map(|r| r.0.iter().find(|e| r.1.contains(e)).unwrap().priority()).sum();

    println!("Sum: {sum}");

    Ok(())
}
