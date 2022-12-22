use anyhow::Result;

use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string(r"Input\Day 1.txt")?;
    let data: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut depth_increases = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            depth_increases += 1;
        }
    }

    println!("Number of depth increases: {depth_increases}");

    Ok(())
}
