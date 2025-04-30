use crate::common::{Relation, parse_relation};
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_2(input: &str) -> i16 {
    // We only use the HashMap for our initial setup
    // to ensure there are no duplicates. This way we
    // can skip slow hashing in our hot loop.
    let mut index_map = HashMap::new();
    let mut persons = Vec::new();
    let mut index = 0;

    // First we parse and store all the relations.
    for line in input.trim().lines() {
        let Relation {
            person,
            neighbor,
            delta,
        } = parse_relation(line.trim()).unwrap().1;

        // Once we have parsed the relation, we save the person
        // into the HashMap storing them with the next available
        // index if they didn't already have one.
        let person_index = *index_map.entry(person).or_insert_with(|| {
            let idx = index;
            index += 1;
            idx
        });

        // Same deal for their neighbor
        let neighbor_index = *index_map.entry(neighbor.clone()).or_insert_with(|| {
            let idx = index;
            index += 1;
            idx
        });

        // For the relation we make sure there is enough space
        // in the relations vector before we index into it with our indices.

        // If the biggest current index wouldn't fit, we make more space
        if persons.len() <= person_index.max(neighbor_index) {
            // Each new spot is filled with a vector
            persons.resize_with(person_index.max(neighbor_index) + 1, Vec::new);
            for likings in persons.iter_mut() {
                // Each spot is then resized to the same number
                // (biggest current index)
                likings.resize(person_index.max(neighbor_index) + 1, 0);
            }
        }

        // If or once there is enough space we store the delta value.
        persons[person_index][neighbor_index] = delta;
    }

    // Add "Jens" with 0 happiness delta to/from everyone
    index += 1;
    for person_neighbors in persons.iter_mut() {
        // Add neighbor with delta of 0 to everyones neighbors
        person_neighbors.push(0);
    }

    // Add a new person "Jens"
    persons.push(vec![0; index]);

    // Create index list
    let indices: Vec<usize> = (0..index).collect();

    indices
        .iter()
        .permutations(index)
        .map(|permutation| {
            let mut total = 0;
            // For each permutation we look at each person `a`
            // and the one next to them `b`
            for i in 0..permutation.len() {
                let a = *permutation[i];
                let b = *permutation[(i + 1) % permutation.len()];

                // We take the total of what both persons think of each other
                total += persons[a][b] + persons[b][a];
            }
            total
        })
        .max()
        .unwrap()
}
