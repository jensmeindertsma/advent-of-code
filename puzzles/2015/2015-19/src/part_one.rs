use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let (replacements, molecule) = input.trim().split_once("\n\n").unwrap();

    let replacements = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .fold(HashMap::new(), |mut map, (from, to)| {
            map.entry(from).or_insert(Vec::new()).push(to);
            map
        });

    let mut distinct = HashSet::new();

    for (from, ends) in replacements {
        for to in ends {
            for (index, _) in molecule.match_indices(from) {
                let prefix = &molecule[..index];
                let suffix = &molecule[index + from.len()..];
                let new_molecule = format!("{prefix}{to}{suffix}");

                distinct.insert(new_molecule);
            }
        }
    }

    distinct.len()
}
