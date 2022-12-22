use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let path = r"Day 5.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let seat_ids: Vec<Result<u32>> = buffered
        .lines()
        .map(|line| {
            let current_line = line?;
            let row = seat_to_decimal(&current_line[0..7]);
            let col = seat_to_decimal(&current_line[7..10]);

            Ok(8 * (row as u32) + (col as u32))
        })
        .collect();

    let mut seat_ids: Vec<u32> = seat_ids.into_iter().flatten().collect();

    seat_ids.sort_unstable();

    for id in 1..seat_ids.len() {
        let (a, b) = (seat_ids[id - 1], seat_ids[id]);
        if b - a == 2 {
            println!("Wanted seat ID: {}", a + 1);
        }
    }

    Ok(())
}

fn seat_to_decimal(input: &str) -> u8 {
    let mut result = 0;

    for (i, character) in input.chars().rev().enumerate() {
        result += match character {
            'B' | 'R' => 2_u8.pow(i as u32),
            _ => 0,
        }
    }

    result
}
