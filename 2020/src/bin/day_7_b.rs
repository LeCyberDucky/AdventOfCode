use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let path = r"Day 7.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut bag_graph: HashMap<String, HashSet<(String, usize)>> = HashMap::new();

    for line in buffered.lines() {
        let current_line = line?;
        let current_line = current_line.replace(".", "");
        let bag_sets: Vec<String> = current_line
            .split("bags contain")
            .map(|s| s.trim().to_string())
            .collect();

        if bag_sets.len() < 2 {
            continue;
        }

        let outer_bag = &bag_sets[0].trim().to_string();
        let inner_bags = &bag_sets[1];

        if inner_bags == "no other bags" {
            // Make sure that outer bag exists on graph
            bag_graph.insert(outer_bag.to_string(), HashSet::new());
            continue;
        }

        // Parse the inner bags
        let inner_bags: Vec<String> = inner_bags
            .split(',')
            .map(|s| {
                s.to_string()
                    .replace("bags", "")
                    .replace("bag", "")
                    .trim()
                    .to_string()
            })
            .collect();

        let bag_neighbors = bag_graph
            .entry(outer_bag.to_string())
            .or_insert_with(HashSet::new);

        for bag in inner_bags {
            let split_point = bag.find(' ').unwrap();
            let key = bag[split_point + 1..bag.len()].to_string();
            let value: usize = bag[0..split_point].parse().unwrap();
            bag_neighbors.insert((key, value));
        }
    }

    let wanted_bag = "shiny gold".to_string();

    let bag_count = bag_sum(&wanted_bag, &bag_graph) - 1;

    println!(
        "The {} bag will need to hold {} individual bags.",
        wanted_bag, bag_count
    );

    Ok(())
}

fn bag_sum(bag: &str, bag_graph: &HashMap<String, HashSet<(String, usize)>>) -> usize {
    if !bag_graph.contains_key(bag) || bag_graph.get(bag).unwrap().is_empty() {
        1
    } else {
        let mut temporary_sum = 1;

        for pair in bag_graph.get(bag).unwrap() {
            temporary_sum += pair.1 * bag_sum(&pair.0, bag_graph);
        }

        temporary_sum
    }
}
