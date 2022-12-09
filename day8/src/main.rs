use std::{error::Error, io::{BufReader, BufRead}};

struct Tree {
    height: usize,
    view_score: usize,
    visible: bool
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("day8/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();
    let mut trees = lines.iter()
        .map(|s| s
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|h| Tree {
                height: h as usize,
                view_score: 1,
                visible: false,
            })
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let height = trees.len();
    let width = trees[0].len();

    for row in 0..height {
        let mut last = vec![None; 10];
        for column in 0..width {
            let tree = &mut trees[row][column];
            let last_higher = last[tree.height..].iter().filter_map(|&o| o).max();
            tree.view_score *= column - last_higher.unwrap_or(0);
            tree.visible |= last_higher.is_none();
            last[tree.height] = Some(column);
        }
        
        let mut last = vec![None; 10];
        for column in (0..width).rev() {
            let tree = &mut trees[row][column];
            let last_higher = last[tree.height..].iter().filter_map(|&o| o).min();
            tree.view_score *= last_higher.unwrap_or(width - 1) - column;
            tree.visible |= last_higher.is_none();
            last[tree.height] = Some(column);
        }
    }

    for column in 0..width {
        let mut last = vec![None; 10];
        for row in 0..height {
            let tree = &mut trees[row][column];
            let last_higher = last[tree.height..].iter().filter_map(|&o| o).max();
            tree.view_score *= row - last_higher.unwrap_or(0);
            tree.visible |= last_higher.is_none();
            last[tree.height] = Some(row);
        }
        
        let mut last = vec![None; 10];
        for row in (0..height).rev() {
            let tree = &mut trees[row][column];
            let last_higher = last[tree.height..].iter().filter_map(|&o| o).min();
            tree.view_score *= last_higher.unwrap_or(height - 1) - row;
            tree.visible |= last_higher.is_none();
            last[tree.height] = Some(row);
        }
    }

    let visible_count = trees.iter().flatten().filter(|t| t.visible).count();
    println!("Visible: {visible_count}");

    let best = trees.iter().flatten().map(|t| t.view_score).max().unwrap();
    println!("Best: {best}");

    Ok(())
}
