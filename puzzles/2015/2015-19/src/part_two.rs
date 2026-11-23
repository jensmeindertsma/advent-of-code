use std::collections::{HashMap, HashSet};

pub fn part_two(input: &str) -> usize {
    let (replacements, molecule) = input.trim().split_once("\n\n").unwrap();

    let replacements = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .fold(HashMap::new(), |mut map, (from, to)| {
            map.entry(from).or_insert(Vec::new()).push(to);
            map
        });

    todo!()
}
