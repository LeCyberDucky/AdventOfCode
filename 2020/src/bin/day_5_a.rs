use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn main() -> Result<()> {
    let path = r"Day 5.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut max_seat_id = 0;

    for line in buffered.lines() {
        let current_line = line?;
        let row = seat_to_decimal(&current_line[0..7]);
        let col = seat_to_decimal(&current_line[7..10]);

        let seat_id = 8 * (row as u32) + (col as u32);

        max_seat_id = if seat_id > max_seat_id {
            seat_id
        } else {
            max_seat_id
        };
    }

    println!("Maximum seat ID: {}", max_seat_id);

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
