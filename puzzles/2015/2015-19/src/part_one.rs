use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let (replacements, molecule) = input.trim().split_once("\n\n").unwrap();

    let replacements = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .fold(HashMap::new(), |mut map, (start, end)| {
            map.entry(start).or_insert(Vec::new()).push(end);
            map
        });

    dbg!(&replacements);

    dbg!(molecule);

    let mut atoms: Vec<&str> = Vec::new();

    // TODO: split molecule into atoms

    let mut distinct = HashSet::new();

    for (start, ends) in replacements {
        for end in ends {
            // TODO: iterate through atoms, replacing each item
            // with every replacement, pushing the result into
            // distinct.

            let new_molecule = String::new();
            distinct.insert(new_molecule);
        }
    }

    distinct.len()
}
