use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    let mut has_double = 0;
    let mut has_triple = 0;

    for line in input.trim().lines() {
        let characters = line.chars();
        let mut counter = HashMap::new();

        for character in characters {
            *counter.entry(character).or_insert(0) += 1;
        }

        if counter.values().any(|v| *v == 2) {
            has_double += 1;
        }

        if counter.values().any(|v| *v == 3) {
            has_triple += 1;
        }
    }

    has_double * has_triple
}
