use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let path = r"Day 3.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut tree_count_multiple = 1;
    let lines: Vec<String> = buffered.lines().map(|s| s.unwrap()).collect();
    let line_count = lines.len();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for slope in slopes.iter() {
        let x_speed = (*slope).0;
        let y_speed = (*slope).1;

        let mut x_position = 0;
        let mut y_position = 0;

        let mut tree_count = 0;

        while y_position < line_count {
            let current_line = &lines[y_position];
            let line_width = current_line.len();
            x_position %= line_width;

            if current_line.chars().nth(x_position).unwrap() == '#' {
                tree_count += 1;
            }

            x_position += x_speed;
            y_position += y_speed;
        }

        tree_count_multiple *= tree_count;
    }

    println!(
        "Product of number of trees on paths: {}",
        tree_count_multiple
    );

    Ok(())
}
