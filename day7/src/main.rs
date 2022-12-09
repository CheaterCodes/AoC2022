use std::{error::Error, io::{BufReader, BufRead}};

struct Directory {
    name: String,
    size: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("day7/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut dir_stack = Vec::new();
    dir_stack.push(Directory {
        name: "".to_owned(),
        size: 0
    });

    let mut completed_dirs = Vec::new();

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
                            let dir = dir_stack.pop().unwrap();
                            let parent = dir_stack.last_mut().unwrap();

                            parent.size += dir.size;
                            completed_dirs.push(dir);
                        }
                    },
                    ".." => {
                        let dir = dir_stack.pop().unwrap();
                        let parent = dir_stack.last_mut().unwrap();

                        parent.size += dir.size;
                        completed_dirs.push(dir);
                    },
                    dir => dir_stack.push(Directory { name: dir.to_owned(), size: 0 }),
                }
            },
            Some("ls") => {
                while let Some(entry) = commands.peek() {
                    let mut parts = entry.split_whitespace();
                    match parts.next() {
                        Some("$") => break,
                        Some("dir") => (),
                        Some(size) => {
                            let dir = dir_stack.last_mut().unwrap();
                            dir.size += size.parse::<usize>().unwrap()
                        },
                        _ => panic!()
                    };
                    commands.next();
                }
            },
            _ => panic!()
        }
    }
    
    while dir_stack.len() > 1 {
        let dir = dir_stack.pop().unwrap();
        let parent = dir_stack.last_mut().unwrap();

        parent.size += dir.size;
        completed_dirs.push(dir);
    }

    let part1: usize = completed_dirs.iter().filter(|dir| dir.size <= 100000).map(|dir| dir.size).sum();
    println!("Part 1: {part1}");

    let total_size = 70000000;
    let used_size = dir_stack[0].size;
    let free_size = total_size - used_size;
    let required_size = 30000000 - free_size;

    let part2 = completed_dirs.iter().filter(|dir| dir.size >= required_size).map(|dir| dir.size).min().unwrap();
    println!("Part 2: {part2}");

    Ok(())
}
