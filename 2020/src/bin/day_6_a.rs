use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let path = r"Day 6.txt";

    let input = read_to_string(path)?;

    let answers: Vec<Vec<char>> = input
        .split("\r\n\r\n")
        .map(|s| s.to_string().replace("\r\n", "").chars().collect())
        .collect();

    let answers: Vec<Vec<char>> = answers
        .into_iter()
        .map(|mut vec| {
            vec.sort();
            vec.dedup();
            vec
        })
        .collect();

    let result: usize = answers.iter().map(|vec| vec.len()).sum();

    println!("The wanted sum is: {}", result);

    Ok(())
}
