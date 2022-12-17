use std::error::Error;

enum Command {
    Left,
    Right
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day17/input.txt")?.trim().chars().collect::<Vec<_>>();
    let mut commands = input.iter().map(|c| if *c == '<' { Command::Left } else { Command::Right }).cycle();

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

    let mut room = Vec::new();

    for rock in rocks.iter().cycle().take(2022) {
        let mut pos = (2, room.len() + 3);
        
        loop {
            match commands.next().unwrap() {
                Command::Left => if pos.0 > 0 && can_place_rock(&mut room, rock, (pos.0 - 1, pos.1)) {
                    pos = (pos.0 - 1, pos.1);
                },
                Command::Right => if can_place_rock(&mut room, rock, (pos.0 + 1, pos.1)) {
                    pos = (pos.0 + 1, pos.1);
                },
            }

            if pos.1 > 0 && can_place_rock(&mut room, rock, (pos.0, pos.1 - 1)) {
                pos = (pos.0, pos.1 - 1);
            } else {
                if !can_place_rock(&mut room, rock, pos) {
                    panic!()
                }
                place_rock(&mut room, rock, pos);
                break;
            }
        }
    }

    println!("height: {}", room.len());

    Ok(())
}

fn can_place_rock(room: &mut Vec<Vec<bool>>, rock: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
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

fn place_rock(room: &mut Vec<Vec<bool>>, rock: &Vec<Vec<bool>>, pos: (usize, usize)) {
    while room.len() < pos.1 + rock.len() {
        room.push(vec![false; 7]);
    }

    for (row, rocks) in rock.iter().rev().enumerate()  {
        for (col, rock) in rocks.iter().enumerate() {
            if *rock {
                room[row + pos.1][col + pos.0] = true;
            }
        }
    }
}

fn print_room(room: &Vec<Vec<bool>>) {
    for level in room.iter().rev() {
        for loc in level {
            if *loc {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}