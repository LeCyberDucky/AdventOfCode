use anyhow::Result;

use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string(r"Input\Day 1.txt")?;
    let data: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut depth_increases = 0;
    for i in 3..data.len() {
        let a: usize = data[i - 3..=i - 1].iter().sum();
        let b: usize = data[i - 2..=i].iter().sum();

        if b > a {
            depth_increases += 1;
        }
    }

    println!("Number of depth increases: {depth_increases}");

    let result = data
        .windows(3)
        .zip(data.windows(3).skip(1))
        .filter(|(a, b)| a.iter().sum::<usize>() < b.iter().sum())
        .count();

    println!("Number of depth increases: {result}");

    Ok(())
}
