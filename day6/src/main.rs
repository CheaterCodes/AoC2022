use std::{error::Error, fs::File, io::{BufReader, BufRead}, collections::VecDeque};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day6/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();
    let mut input = lines[0].chars();

    let mut start_marker = VecDeque::new();
    start_marker.extend((&mut input).take(3));

    let mut taken = 3;
    for char in &mut input {
        taken += 1;

        start_marker.push_back(char);
        if is_marker(start_marker.iter().copied()) {
            break;
        }

        start_marker.pop_front();
    }

    println!("Packet starts at {taken}");

    Ok(())
}

fn is_marker(marker: impl IntoIterator<Item = char>) -> bool {
    let mut seen = Vec::new();
    for c in marker {
        if seen.contains(&c) {
            return false;
        }

        seen.push(c);
    }

    true
}
