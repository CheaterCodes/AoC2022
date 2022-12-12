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
    distance[goal.0][goal.1] = 0;

    let mut to_check = VecDeque::new();
    to_check.push_back(goal);
    while let Some(current) = to_check.pop_front() {
        let current_distance = distance[current.0][current.1];
        for next in get_previous(&height_map, current) {
            let next_distance = &mut distance[next.0][next.1];
            if *next_distance > current_distance + 1 {
                *next_distance = current_distance + 1;
                to_check.push_back(next);
            }
        }
    }

    let start_distance = distance[start.0][start.1];
    println!("Part 1: {start_distance}");

    let min_distance = height_map.iter().flatten().zip(distance.iter().flatten())
        .filter_map(|(&height, &distance)| (height == 0).then_some(distance))
        .min().unwrap();
    println!("Part 2: {min_distance}");

    Ok(())
}

fn get_previous(height_map: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
    let max_row = height_map.len() - 1;
    let max_col = height_map[0].len() - 1;
    let center_height = height_map[row][col];

    (0..4).into_iter().filter_map(move |dir| {
        match dir {
            0 => if row == 0 { None } else { Some((row - 1,  col)) },
            1 => if row == max_row { None } else { Some((row + 1,  col)) },
            2 => if col == 0 { None } else { Some((row,  col - 1)) },
            3 => if col == max_col { None } else { Some((row,  col + 1)) },
            _ => unreachable!()
        }
    }).filter(move |(row, col)| height_map[*row][*col] + 1 >= center_height)
}
