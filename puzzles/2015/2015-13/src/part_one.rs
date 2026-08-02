use crate::parsing;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> i64 {
    let (guests, relations) = input
        .trim()
        .lines()
        .map(parsing::relation)
        .map(|result| result.expect("every line should describe a valid relation"))
        .fold(
            (HashSet::new(), HashMap::new()),
            |(mut guests, mut relations), (person_a, person_b, happiness)| {
                guests.insert(person_a);
                guests.insert(person_b);

                relations.insert((person_a, person_b), happiness);

                (guests, relations)
            },
        );

    let guest_count = guests.len();

    guests
        .into_iter()
        .permutations(guest_count)
        .map(|mut arrangement| {
            let first = arrangement
                .first()
                .expect("there should be guests in the arrangements");

            arrangement.push(first);

            arrangement
                .into_iter()
                .tuple_windows()
                .map(|(person_a, person_b)| {
                    let a_to_b = relations
                        .get(&(person_a, person_b))
                        .expect("every relation should be in the map");

                    let b_to_a = relations
                        .get(&(person_b, person_a))
                        .expect("every relation should be in the map");

                    a_to_b + b_to_a
                })
                .sum()
        })
        .max()
        .expect("there should always be a seating arrangement")
}
