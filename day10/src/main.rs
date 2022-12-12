use std::{fs::File, error::Error, io::{BufReader, BufRead}};

enum Command {
    NoOp,
    AddX(i32),
}

impl Command {
    fn cycles(&self) -> i32 {
        match self {
            Command::NoOp => 1,
            Command::AddX(_) => 2,
        }
    }

    fn perform(&self, x: &mut i32) {
        match self {
            Command::NoOp => (),
            Command::AddX(val) => *x += val,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day10/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap);
    let commands = lines.map(|line| {
        if line == "noop" {
            Command::NoOp
        } else {
            Command::AddX(line[5..].parse::<i32>().unwrap())
        }
    });

    let mut cycle = 0;
    let mut reg_x = 1;

    let mut sum_signal = 0;

    for command in commands {
        for _ in 0..command.cycles() {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum_signal += reg_x * cycle;
            }
        }

        command.perform(&mut reg_x);
    }

    println!("Part 1: {sum_signal}");

    Ok(())
}
