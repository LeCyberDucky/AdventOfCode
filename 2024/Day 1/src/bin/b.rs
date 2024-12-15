use parse_display::FromStr;
use std::{collections::{HashMap, HashSet}, fs::read_to_string};

#[derive(FromStr, Debug)]
#[display("{left}   {right}")]
struct Line {
    left: u32,
    right: u32,
}

fn main() {
    // let input = r"example input.txt";
    let input = r"input.txt";
    let input = read_to_string(input).expect("Failed to read input").trim().to_string();
    let data: Vec<Line> = input
        .split('\n')
        .map(|x| x.trim().parse().expect("Failed to parse input"))
        .collect();

    let mut left = vec![];
    let mut right = HashMap::new();

    for line in data {
        left.push(line.left);
        *right.entry(line.right).or_insert(0) += 1;
    }

    let mut sum = 0;
    for value in left.iter() {
        sum += value * right.get(value).unwrap_or(&0);
    }

    println!("Sum: {sum}");
}
