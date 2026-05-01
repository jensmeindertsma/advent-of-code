use crate::parsing;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let (paths, cities) = input
        .trim()
        .lines()
        .map(parsing::description)
        .map(|result| result.expect("each description should be valid"))
        .fold(
            (HashMap::new(), HashSet::new()),
            |(mut paths, mut cities), (from, to, distance)| {
                paths.insert((from, to), distance);
                paths.insert((to, from), distance);

                cities.insert(from);
                cities.insert(to);

                (paths, cities)
            },
        );

    cities
        .iter()
        .permutations(cities.len())
        .map(|route| {
            route
                .iter()
                .tuple_windows()
                .map(|(from, to)| {
                    paths
                        .get(&(from, to))
                        .expect("every path should be present")
                })
                .map(|n| *n as usize)
                .sum()
        })
        .min()
        .expect("there should always be routes")
}
