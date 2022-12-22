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

        let min_count: usize = parts[0].parse()?;
        let max_count: usize = parts[1].parse()?;
        let rule = &parts[2];
        let password = &parts[4];

        let rule_count = password.matches(rule);
        let rule_count = &rule_count.count();

        if (*rule_count >= min_count && *rule_count <= max_count) {
            valid_passwords += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_passwords);

    Ok(())
}
