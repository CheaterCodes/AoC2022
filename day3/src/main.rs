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
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

    let duplicate_priority_sum = lines.iter()
        .map(|l| {
            let parts = l.split_at(l.len() / 2);
            (parts.0.chars().collect::<Vec<_>>(), parts.1.chars().collect::<Vec<_>>())
        })
        .map(|r| r.0.iter().find(|e| r.1.contains(e)).unwrap().priority())
        .sum::<i32>();

    println!("Sum: {duplicate_priority_sum}");

    let mut badge_iter = lines.iter();
    let mut badge_sum = 0;
    while let (Some(a), Some(b), Some(c)) = (badge_iter.next(), badge_iter.next(), badge_iter.next()) {
        let badge = a.chars().find(|item| b.contains(*item) && c.contains(*item)).unwrap();
        badge_sum += badge.priority();
    }
    
    println!("Badge Sum: {badge_sum}");

    Ok(())
}
