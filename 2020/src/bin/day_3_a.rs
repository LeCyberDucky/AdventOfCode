use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn main() -> Result<()> {
    let path = r"Day 3.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut tree_count = 0;
    let mut x_position = 0;

    for line in buffered.lines() {
        let current_line = line?;
        let line_width = current_line.len();
        x_position = x_position % line_width;

        if (current_line.chars().nth(x_position).unwrap() == '#') {
            tree_count += 1;
        }

        x_position += 3;
    }

    println!("Number of trees on path: {}", tree_count);

    Ok(())
}
