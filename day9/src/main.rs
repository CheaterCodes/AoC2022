use std::{error::Error, io::{BufReader, BufRead}, collections::BTreeSet};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("day9/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap);
    let commands = lines.map(|l| {
        let dir = match l.chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("Unexpected char: {c}")
        };
        let count = l[2..].parse::<usize>().unwrap();
        std::iter::repeat(dir).take(count)
    }).flatten();

    let mut knots = vec![(0, 0); 10];

    let mut visited_part1 = BTreeSet::new();
    visited_part1.insert((0, 0));

    let mut visited_part2 = BTreeSet::new();
    visited_part2.insert((0, 0));

    for dir in commands {
        let head = &mut knots[0];
        match dir {
            Direction::Up => head.1 -= 1,
            Direction::Down => head.1 += 1,
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
        }

        for i in 1..knots.len() {
            let prev = knots[i - 1];
            let knot = &mut knots[i];

            if i32::abs_diff(prev.0, knot.0) > 1 || i32::abs_diff(prev.1, knot.1) > 1 {
                knot.0 += (prev.0 - knot.0).signum();
                knot.1 += (prev.1 - knot.1).signum();
            } else {
                break;
            }
        }

        visited_part1.insert(knots[1]);
        visited_part2.insert(*knots.last().unwrap());
    }

    println!("Part 1: {}", visited_part1.len());
    println!("Part 2: {}", visited_part2.len());

    Ok(())
}
