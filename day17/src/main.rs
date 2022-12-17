use std::{error::Error, collections::{VecDeque, HashSet}, iter::Peekable};

#[derive(Clone, Copy)]
enum Command {
    Left,
    Right
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day17/input.txt")?.trim().chars().collect::<Vec<_>>();
    let commands = input.iter().map(|c| if *c == '<' { Command::Left } else { Command::Right }).collect::<Vec<_>>();

    let rocks = [
        vec![
            vec![true, true, true, true]
        ],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![
            vec![true],
            vec![true],
            vec![true],
            vec![true],
        ],
        vec![
            vec![true, true],
            vec![true, true],
        ],
    ];
    
    let part_1 = find_height(&rocks, &commands, 2022);
    let part_2 = find_height(&rocks, &commands, 1000000000000);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn find_height(rocks: &[Vec<Vec<bool>>], commands: &Vec<Command>, num_rocks: i64) -> i64 {
    let mut commands = commands.iter().copied().enumerate().cycle().peekable();

    let mut room = VecDeque::new();

    let base_cycle = run_cycle(&mut room, &rocks, &mut commands);
    let base_height = room.len() as i64;

    let cycle_length = run_cycle(&mut room, &rocks, &mut commands);
    let cycle_height = room.len() as i64 - base_height;

    let top_length = (num_rocks - base_cycle) % cycle_length;
    run_n(&mut room, &rocks, &mut commands, top_length as usize);
    let top_height = room.len() as i64 - cycle_height - base_height;

    let total = base_height + cycle_height * ((num_rocks - base_cycle) / cycle_length) + top_height;

    total
}

fn run_cycle(room: &mut VecDeque<Vec<bool>>, rocks: &[Vec<Vec<bool>>], commands: &mut Peekable<impl Iterator<Item = (usize, Command)>>) -> i64 {
    let mut cycle_length = 0;

    let mut known_locations = HashSet::new();

    for (rock_id, rock) in rocks.iter().enumerate().cycle() {
        let mut pos = (2, room.len() + 3);
        
        if rock_id == 0 {
            let command_id = commands.peek().unwrap().0;
            if !known_locations.insert(command_id) {
                return cycle_length;
            }
        }

        cycle_length += 1;
        
        loop {
            match commands.next().unwrap().1 {
                Command::Left => if pos.0 > 0 && can_place_rock(room, rock, (pos.0 - 1, pos.1)) {
                    pos = (pos.0 - 1, pos.1);
                },
                Command::Right => if can_place_rock(room, rock, (pos.0 + 1, pos.1)) {
                    pos = (pos.0 + 1, pos.1);
                },
            }

            if pos.1 > 0 && can_place_rock(room, rock, (pos.0, pos.1 - 1)) {
                pos = (pos.0, pos.1 - 1);
            } else {
                if !can_place_rock(room, rock, pos) {
                    panic!()
                }
                place_rock(room, rock, pos);
                break;
            }
        }
    }

    unreachable!()
}

fn run_n(room: &mut VecDeque<Vec<bool>>, rocks: &[Vec<Vec<bool>>], commands: &mut Peekable<impl Iterator<Item = (usize, Command)>>, n: usize) {
    for rock in rocks.iter().cycle().take(n) {
        let mut pos = (2, room.len() + 3);
        
        loop {
            match commands.next().unwrap().1 {
                Command::Left => if pos.0 > 0 && can_place_rock(room, rock, (pos.0 - 1, pos.1)) {
                    pos = (pos.0 - 1, pos.1);
                },
                Command::Right => if can_place_rock(room, rock, (pos.0 + 1, pos.1)) {
                    pos = (pos.0 + 1, pos.1);
                },
            }

            if pos.1 > 0 && can_place_rock(room, rock, (pos.0, pos.1 - 1)) {
                pos = (pos.0, pos.1 - 1);
            } else {
                if !can_place_rock(room, rock, pos) {
                    panic!()
                }
                place_rock(room, rock, pos);
                break;
            }
        }
    }
}

fn can_place_rock(room: &mut VecDeque<Vec<bool>>, rock: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    for (row, rocks) in rock.iter().rev().enumerate()  {
        for (col, rock) in rocks.iter().enumerate() {
            if col + pos.0 >= 7 {
                return false;
            }

            if row + pos.1 >= room.len() {
                continue;
            }

            if *rock && room[row + pos.1][col + pos.0] {
                return false;
            }
        }
    }

    return true;
}

fn place_rock(room: &mut VecDeque<Vec<bool>>, rock: &Vec<Vec<bool>>, pos: (usize, usize)) {
    while room.len() < pos.1 + rock.len() {
        room.push_back(vec![false; 7]);
    }

    for (row, rocks) in rock.iter().rev().enumerate()  {
        for (col, rock) in rocks.iter().enumerate() {
            if *rock {
                room[row + pos.1][col + pos.0] = true;
            }
        }
    }
}
