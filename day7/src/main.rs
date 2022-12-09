use std::{error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("day7/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut dir_stack = Vec::new();
    dir_stack.push(0);

    let mut total_size = 0;

    let mut commands = lines.iter().peekable();
    while let Some(command) = commands.next() {
        let mut parts = command.split_whitespace();
        assert!(parts.next() == Some("$"));
        match parts.next() {
            Some("cd") => {
                match parts.next().unwrap() {
                    "/" => {
                        while dir_stack.len() > 1 {
                            let size = dir_stack.pop().unwrap();
                            if size <= 100000 {
                                total_size += size;
                            }
                            *dir_stack.last_mut().unwrap() += size;
                        }
                    },
                    ".." => {
                        let size = dir_stack.pop().unwrap();
                        if size <= 100000 {
                            total_size += size;
                        }
                        *dir_stack.last_mut().unwrap() += size;
                    },
                    _dir => dir_stack.push(0),
                }
            },
            Some("ls") => {
                while let Some(entry) = commands.peek() {
                    let mut parts = entry.split_whitespace();
                    match parts.next() {
                        Some("$") => break,
                        Some("dir") => (),
                        Some(size) => *dir_stack.last_mut().unwrap() += size.parse::<usize>().unwrap(),
                        _ => panic!()
                    };
                    commands.next();
                }
            },
            _ => panic!()
        }

        println!("{dir_stack:?}");
    }
    
    while dir_stack.len() > 1 {
        let size = dir_stack.pop().unwrap();
        if size <= 100000 {
            total_size += size;
        }
        *dir_stack.last_mut().unwrap() += size;
    }

    println!("{total_size}");

    Ok(())
}
