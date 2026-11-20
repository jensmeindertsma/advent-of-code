use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    let (replacements, molecule) = input.trim().split_once("\n\n").unwrap();

    let replacements = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .fold(HashMap::new(), |mut map, (start, end)| {
            map.entry(start).or_insert(Vec::new()).push(end);
            map
        });

    println!("{replacements:#?}");

    println!("`{molecule}`");

    1
}
