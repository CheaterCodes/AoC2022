use std::{error::Error, collections::HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day18/input.txt")?;
    
    let mut points = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();

        points.push((x, y, z));
    }

    let mut checked = HashSet::new();
    let mut surface = 0;
    for (x, y, z) in points {
        checked.insert((x, y, z));
        surface += 6;

        if checked.contains(&(x + 1, y, z)) { surface -= 2 }
        if checked.contains(&(x - 1, y, z)) { surface -= 2 }
        if checked.contains(&(x, y + 1, z)) { surface -= 2 }
        if checked.contains(&(x, y - 1, z)) { surface -= 2 }
        if checked.contains(&(x, y, z + 1)) { surface -= 2 }
        if checked.contains(&(x, y, z - 1)) { surface -= 2 }
    }

    println!("Part 1: {surface}");

    Ok(())
}
