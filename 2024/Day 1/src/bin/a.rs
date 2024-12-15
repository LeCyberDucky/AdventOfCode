use parse_display::FromStr;
use std::fs::read_to_string;

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
    let mut right = vec![];

    for line in data {
        left.push(line.left);
        right.push(line.right);
    }

    left.sort();
    right.sort();

    let sum_of_deltas: u32 = left.iter().zip(right.iter()).map(|(a, b)| a.abs_diff(*b)).sum();

    println!("{:?}", sum_of_deltas);
}
