use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
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

    let mut modified_instruction = 0;
    let mut modified = false;
    let mut instruction = 0;
    let mut accumulator = 0;

    let instruction_count = instructions.len();
    println!("Instruction count: {}", instruction_count);

    loop {
        if modified {
            match instructions[modified_instruction].0 {
                Operation::Jmp(offset) => instructions[modified_instruction].0 = Operation::Nop(offset),
                Operation::Nop(offset) => instructions[modified_instruction].0 = Operation::Jmp(offset),
                Operation::Acc(offset) => instructions[modified_instruction].0 = Operation::Acc(offset),
            }

            modified_instruction += 1;
        } else {
            modified = true;
        }

        let mut instructions: Vec<(Operation, bool)> = instructions.iter().map(|pair| (pair.0, false)).collect();

        while let Operation::Acc(_) = instructions[modified_instruction].0 {
            modified_instruction += 1;
        }

        match instructions[modified_instruction].0 {
            Operation::Jmp(offset) => instructions[modified_instruction].0 = Operation::Nop(offset),
            Operation::Nop(offset) => instructions[modified_instruction].0 = Operation::Jmp(offset),
            Operation::Acc(offset) => instructions[modified_instruction].0 = Operation::Acc(offset),
        }

        // Reset to start new simulation with modified instructions
        accumulator = 0;
        instruction = 0;
    
        loop {
            if instruction >= instructions.len() {
                break;
            }

            let (op, visited) = &mut instructions[instruction];
    
            if *visited {
                break;
            }
    
            *visited = true;
    
            match op {
                Operation::Acc(offset) => {
                    accumulator += *offset;
                    instruction += 1
                }
                Operation::Jmp(offset) => instruction = (instruction as isize + *offset) as usize,
                Operation::Nop(_offset) => instruction += 1,
            }
        }

        if instruction == instruction_count - 1 {
            let (op, visited) = &mut instructions[instruction];

            match op {
                Operation::Acc(offset) => {
                    accumulator += *offset;
                    instruction += 1
                }
                Operation::Jmp(offset) => instruction = (instruction as isize + *offset) as usize,
                Operation::Nop(_offset) => instruction += 1,
            }

            println!("Alter...");
            break;
        }

        println!("Modified insruction: {} | Instruction: {} | Accumulator value: {}", modified_instruction, instruction, accumulator);
    }


    println!(
        "Accumulator value after termination of fixed program: {}",
        accumulator
    );

    Ok(())
}
