use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let path = r"Day 7.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut bag_graph = HashMap::new();

    // Build bag graph by adding the outer bag as a neighbor to all its inner bags
    for line in buffered.lines() {
        let current_line = line?;
        let current_line = current_line.replace(".", "");
        let bag_sets: Vec<String> = current_line
            .split("bags contain")
            .map(|s| s.to_string())
            .collect();

        if bag_sets.len() < 2 {
            continue;
        }

        let outer_bag = &bag_sets[0].trim().to_string();
        let inner_bags = &bag_sets[1];

        if inner_bags == "no other bags" {
            continue;
        }

        let inner_bags: Vec<String> = inner_bags
            .split(',')
            .map(|s| {
                s.to_string()
                    .replace(|ch: char| ch.is_ascii_digit(), "")
                    .replace("bags", "")
                    .replace("bag", "")
                    .trim()
                    .to_string()
            })
            .collect();

        // Add outer bag as neighbor to inner bags
        for bag in inner_bags {
            let bag_neighbors = bag_graph.entry(bag).or_insert_with(HashSet::new);
            bag_neighbors.insert(outer_bag.to_string());
        }
    }

    // Breadth first search from wanted inner bag (or depth first? dunno)
    let wanted_bag = "shiny gold";

    let mut visited_bags = HashSet::new();
    visited_bags.insert(wanted_bag.to_string());

    let mut bags_to_visit: VecDeque<String> = bag_graph
        .get(wanted_bag)
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect();

    while !bags_to_visit.is_empty() {
        let current_bag = &bags_to_visit.pop_front().unwrap();
        if visited_bags.contains(current_bag) {
            continue;
        }

        visited_bags.insert(current_bag.to_string());

        if !bag_graph.contains_key(current_bag) {
            // This bag has no outer bags
            continue;
        }

        for bag in bag_graph.get(current_bag).unwrap() {
            bags_to_visit.push_back(bag.to_string());
        }
    }

    // Remember to subtract the wanted bag from the wisited bags. Shouldn't contain itself.
    println!("Visited bags:\n{:#?}", visited_bags);
    println!(
        "{} bag colors can eventually contain at least one {} bag.",
        visited_bags.len() - 1,
        wanted_bag
    );

    Ok(())
}
