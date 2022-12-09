use std::{error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("day8/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();
    let heights = lines.iter().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let height = heights.len();
    let width = heights[0].len();

    let mut visible = vec![vec![false; width]; height];

    for row in 0..height {
        visible[row][0] = true;
        visible[row][width - 1] = true;

        let mut max = heights[row][0];
        for column in 1 .. width - 1 {
            if heights[row][column] > max {
                max = heights[row][column];
                visible[row][column] = true;
            }
        }
        
        let mut max = heights[row][width - 1];
        for column in (1 .. width - 1).rev() {
            if heights[row][column] > max {
                max = heights[row][column];
                visible[row][column] = true;
            }
        }
    }
    
    for column in 0..height {
        visible[0][column] = true;
        visible[height - 1][column] = true;

        let mut max = heights[0][column];
        for row in 1 .. height - 1 {
            if heights[row][column] > max {
                max = heights[row][column];
                visible[row][column] = true;
            }
        }
        
        let mut max = heights[0][column];
        for row in (1 .. height - 1).rev() {
            if heights[row][column] > max {
                max = heights[row][column];
                visible[row][column] = true;
            }
        }
    }

    let visible_count = visible.iter().flatten().filter(|&&v| v).count();
    println!("Visible: {visible_count}");

    Ok(())
}
