use std::{error::Error, collections::HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day18/input.txt")?;
    
    let mut lava = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();

        lava.insert((x, y, z));
    }

    println!("Part 1: {}", get_surface(&lava));

    let min = lava.iter().map(|(x, y, z)| x.min(y.min(z))).min().unwrap() - 1;
    let max = lava.iter().map(|(x, y, z)| x.max(y.max(z))).max().unwrap() + 1;
    let mut water = HashSet::new();

    flood_fill(&mut water, &lava, min, max, (min, min, min));

    let mut droplet = HashSet::new();
    for &(x, y, z) in &lava {
        flood_fill(&mut droplet, &water, min, max, (x, y, z));
    }

    println!("Part 2: {}", get_surface(&droplet));

    Ok(())
}

fn flood_fill(fill: &mut HashSet<(i64, i64, i64)>, barrier: &HashSet<(i64, i64, i64)>, min: i64, max: i64, (x, y, z): (i64, i64, i64)) {
    if barrier.contains(&(x, y, z)) || x < min || y < min || z < min || x > max || y > max || z > max {
        return;
    }

    if fill.insert((x, y, z)) {
        flood_fill(fill, barrier, min, max, (x + 1, y, z));
        flood_fill(fill, barrier, min, max, (x - 1, y, z));
        flood_fill(fill, barrier, min, max, (x, y + 1, z));
        flood_fill(fill, barrier, min, max, (x, y - 1, z));
        flood_fill(fill, barrier, min, max, (x, y, z + 1));
        flood_fill(fill, barrier, min, max, (x, y, z - 1));
    }
}

fn get_surface(volume: &HashSet<(i64, i64, i64)>) -> i64 {
    let mut surface = 0;

    for &(x, y, z) in volume {
        if !volume.contains(&(x + 1, y, z)) { surface += 1 }
        if !volume.contains(&(x - 1, y, z)) { surface += 1 }
        if !volume.contains(&(x, y + 1, z)) { surface += 1 }
        if !volume.contains(&(x, y - 1, z)) { surface += 1 }
        if !volume.contains(&(x, y, z + 1)) { surface += 1 }
        if !volume.contains(&(x, y, z - 1)) { surface += 1 }
    }

    surface
}