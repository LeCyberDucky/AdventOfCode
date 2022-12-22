use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Operation {
    fn new(input: String) -> Result<Operation> {
        let parts: Vec<&str> = input.split(' ').collect();

        match parts[0] {
            "acc" => {
                let index = parts[1].parse()?;
                Ok(Operation::Acc(index))
            }
            "jmp" => {
                let index = parts[1].parse()?;
                Ok(Operation::Jmp(index))
            }
            "nop" => {
                let index = parts[1].parse()?;
                Ok(Operation::Nop(index))
            }
            _ => anyhow::bail!("Dang!"),
        }
    }
}

fn main() -> Result<()> {
    let path = r"Day 8.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut instructions: Vec<(Operation, bool)> = buffered
        .lines()
        .flatten()
        .map(Operation::new)
        .flatten()
        .map(|op| (op, false))
        .collect();

    let mut accumulator = 0;
    let mut i = 0;

    loop {
        let (op, visited) = &mut instructions[i];

        if *visited {
            break;
        }

        *visited = true;

        match op {
            Operation::Acc(offset) => {
                accumulator += *offset;
                i += 1
            }
            Operation::Jmp(offset) => i = (i as isize + *offset) as usize,
            Operation::Nop(_offset) => i += 1,
        }
    }

    println!(
        "Accumulator value before entering infinite loop: {}",
        accumulator
    );

    Ok(())
}
