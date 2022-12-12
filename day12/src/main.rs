use std::{error::Error, fs::File, io::{BufReader, BufRead}, collections::VecDeque};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day12/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap);
    let char_map = lines.map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut height_map = vec![vec![0; char_map[0].len()]; char_map.len()];

    for row in 0..char_map.len() {
        for column in 0..char_map[row].len() {
            let mut char = char_map[row][column];
            
            if char == 'S' {
                start = (row, column);
                char = 'a';
            }

            if char == 'E' {
                goal = (row, column);
                char = 'z';
            }

            height_map[row][column] = char as u8 - 'a' as u8;
        }
    }

    let mut distance = vec![vec![usize::MAX; char_map[0].len()]; char_map.len()];
    distance[start.0][start.1] = 0;

    let mut to_check = VecDeque::new();
    to_check.push_back(start);
    while let Some(current) = to_check.pop_front() {
        let current_distance = distance[current.0][current.1];
        for next in get_next(&height_map, current) {
            let next_distance = &mut distance[next.0][next.1];
            if *next_distance > current_distance + 1 {
                *next_distance = current_distance + 1;
                to_check.push_back(next);
            }
        }
    }

    let goal_distance = distance[goal.0][goal.1];
    println!("Part 1: {goal_distance}");

    Ok(())
}

fn get_next(height_map: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
    let max_row = height_map.len() - 1;
    let max_col = height_map[0].len() - 1;
    let center_height = height_map[row][col];

    Direction::ALL.iter().filter_map(move |dir| {
        match dir {
            Direction::Up => if row == 0 { None } else { Some((row - 1,  col)) },
            Direction::Down => if row == max_row { None } else { Some((row + 1,  col)) },
            Direction::Left => if col == 0 { None } else { Some((row,  col - 1)) },
            Direction::Right => if col == max_col { None } else { Some((row,  col + 1)) },
        }
    }).filter(move |(row, col)| height_map[*row][*col] <= center_height + 1)
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
}
