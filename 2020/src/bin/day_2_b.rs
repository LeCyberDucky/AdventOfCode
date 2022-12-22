use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn main() -> Result<()> {
    let path = r"Day 2.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut valid_passwords = 0;

    for line in buffered.lines() {
        let parts: Vec<String> = line?
            .split([':', '-', ' '].as_ref())
            .map(|s| s.to_string())
            .collect();

        let mut pos_a: usize = parts[0].parse()?;
        pos_a -= 1;
        let mut pos_b: usize = parts[1].parse()?;
        pos_b -= 1;
        let letter = parts[2].chars().next().unwrap();
        let password = &parts[4];
        let password_length = password.len();

        if (pos_a >= 0 && pos_a < password_length && pos_b >= 0 && pos_b < password_length) {
            let pos_a_match = password.chars().nth(pos_a).unwrap() == letter;
            let pos_b_match = password.chars().nth(pos_b).unwrap() == letter;

            if (pos_a_match != pos_b_match) {
                valid_passwords += 1;
            }
        }
    }

    println!("Number of valid passwords: {}", valid_passwords);

    Ok(())
}
