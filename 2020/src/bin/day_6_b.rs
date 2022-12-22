use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let path = r"Day 6.txt";

    let input = read_to_string(path)?;
    let input = input.trim().to_string(); // Yup, this is necessary.

    let answers: Vec<String> = input.split("\r\n\r\n").map(|s| s.to_string()).collect();

    let answers: Vec<Vec<String>> = answers
        .iter()
        .map(|s| s.split("\r\n").map(|s| s.to_string()).collect())
        .collect();

    let answers: Vec<Vec<Vec<char>>> = answers
        .iter()
        .map(|vec| vec.iter().map(|s| s.chars().collect()).collect())
        .collect();

    let answers: Vec<Vec<i32>> = answers
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|answer| answers_as_bits(answer.to_vec()))
                .collect()
        })
        .collect();

    let answers: Vec<i32> = answers
        .into_iter()
        .map(|group| {
            let mut group_answer = !0;
            for answer in group {
                group_answer &= answer;
            }
            group_answer
        })
        .collect();

    let answers: Vec<u32> = answers
        .into_iter()
        .map(|group_answer| group_answer.count_ones())
        .collect();

    let answer_sum: u32 = answers.iter().sum();

    println!("Wanted sum: {}", answer_sum);

    Ok(())
}

fn answers_as_bits(answers: Vec<char>) -> i32 {
    let mut result = 0;

    for character in answers {
        let bit = character as i32 - 'a' as i32;

        result |= 1 << bit;
    }

    result
}
