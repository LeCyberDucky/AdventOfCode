// n^2 solution: For every input number, loop over all the others to check for a match
// n*log(n) solution: Sort the numbers and check for pairs at the front and end or something like that
// n solution: Iterate over the input and for every number, mark its spot in an array and check if its counter part has been encountered

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn main() -> Result<()> {
    let path = r"Day 1.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    // Assuming the input can't be negative, we only need to check numbers between 0 and 2020
    let mut encountered_numbers = vec![false; 2021];

    for line in buffered.lines() {
        let number = line?.trim().parse::<usize>()?;

        encountered_numbers[number] = true;

        if (encountered_numbers[2020 - number]) {
            println!(
                "{} + {} = {}",
                number,
                2020 - number,
                number + (2020 - number)
            );
            println!(
                "{} * {} = {}",
                number,
                2020 - number,
                number * (2020 - number)
            );
        }
    }

    Ok(())
}
