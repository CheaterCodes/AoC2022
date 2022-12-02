use std::{fs::File, io::{BufReader, BufRead}, error::Error};

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(value: &str) -> Choice {
        match value {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!()
        }
    }

    fn get_better(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
    
    fn get_worse(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn from_str(value: &str) -> RoundResult {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!()
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day2/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap);


    let mut sum = 0;
    for line in lines {
        let mut parts = line.split_whitespace();
        let opponent = Choice::from_str(parts.next().unwrap());
        let result = RoundResult::from_str(parts.next().unwrap());

        let choice = match result {
            RoundResult::Win => opponent.get_better(),
            RoundResult::Lose => opponent.get_worse(),
            RoundResult::Draw => opponent,
        };

        let score = result.score() + choice.score();
        sum += score;
    }

    println!("Score: {sum}");

    Ok(())
}
