use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day5/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();
    let mut parts = lines.split(|line| line.is_empty());
    
    let stack_input = parts.next().unwrap().iter()
        .rev()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        ;

    let num_stacks = (stack_input[0].len() + 1) / 4;
    let mut stacks = Vec::new();

    for stack_index in 0..num_stacks {
        let stack = stack_input[1..].iter()
            .map(|line| line[stack_index * 4 + 1])
            .take_while(char::is_ascii_uppercase)
            .collect::<Vec<_>>();

        stacks.push(stack);
    }

    let mut stacks_part_1 = stacks.clone();
    let mut stacks_part_2 = stacks;

    for command in parts.next().unwrap().iter() {
        let mut parts = command.split_ascii_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            ;
        
        let amount = parts.next().unwrap();
        let source = parts.next().unwrap() - 1;
        let destination = parts.next().unwrap() - 1;

        for _ in 0..amount {
            let top = stacks_part_1[source].pop().unwrap();
            stacks_part_1[destination].push(top);
        }

        let source = &mut stacks_part_2[source];
        let to_move = source.drain(source.len() - amount ..).collect::<Vec<_>>();

        let destination = &mut stacks_part_2[destination];
        destination.extend_from_slice(&to_move);
    }

    for stack in stacks_part_1 {
        print!("{}", stack.last().unwrap());
    }
    println!();

    for stack in stacks_part_2 {
        print!("{}", stack.last().unwrap());
    }
    println!();

    Ok(())
}
