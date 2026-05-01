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
            // Vec has length of number of cities, so we need to do
            // windows overlapping to sum the number of distances
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

mod parsing {
    use nom::{
        Finish, Parser,
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::{map, map_res},
    };

    pub fn description(input: &str) -> Option<(&str, &str, u16)> {
        map(
            (destination, tag(" to "), destination, tag(" = "), distance),
            |(from, _, to, _, distance)| (from, to, distance),
        )
        .parse(input)
        .finish()
        .ok()
        .map(|(_, result)| result)
    }

    fn destination(input: &str) -> nom::IResult<&str, &str> {
        alpha1.parse(input)
    }

    fn distance(input: &str) -> nom::IResult<&str, u16> {
        map_res(digit1, str::parse).parse(input)
    }
}
