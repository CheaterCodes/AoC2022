use std::{error::Error, fs::File, io::{BufReader, BufRead}, collections::BTreeSet};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day14/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader.lines()
        .map(Result::unwrap)
        .map(|line| line.split(" -> ")
            .map(|pair| {
                let mut parts = pair.splitn(2, ',');
                (
                    parts.next().unwrap().parse::<i64>().unwrap(),
                    parts.next().unwrap().parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map = BTreeSet::new();
    for path in &input {
        let mut start = path[0];
        map.insert(start);

        for &end in &path[1..] {
            while start.0 != end.0 {
                start.0 += (end.0 - start.0).signum();
                map.insert(start);
            }
            while start.1 != end.1 {
                start.1 += (end.1 - start.1).signum();
                map.insert(start);
            }
        }
    }

    let source = (500, 0);
    let max_y = input.iter().flatten().map(|p| p.1).max().unwrap();

    let mut total_sand = 0;
    let mut part1 = None;
    loop {
        let sand = add_sand(&map, source, max_y + 1);
        map.insert(sand);

        if sand.1 > max_y && part1.is_none() {
            part1 = Some(total_sand);
            println!("Part 1: {total_sand}");
        }

        total_sand += 1;

        if sand == source {
            break;
        }
    }

    println!("Part 2: {total_sand}");

    Ok(())
}

fn add_sand(map: &BTreeSet<(i64, i64)>, start: (i64, i64), max_y: i64) -> (i64, i64) {
    let mut sand = start;
    while sand.1 < max_y {
        if !map.contains(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
        } else if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1);
        } else if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1);
        } else {
            break;
        }
    }

    sand
}