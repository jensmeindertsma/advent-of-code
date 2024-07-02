use itertools::Itertools;
use std::{collections::HashMap, ops::Deref, str::FromStr};

pub fn part_1(input: &str) -> usize {
    let route_tree = build_route_tree(input.trim().lines().map(|line| {
        let route: Route = line.trim().parse().unwrap();
        route
    }));
    *calculate_route_distances(route_tree).iter().min().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let route_tree = build_route_tree(input.trim().lines().map(|line| {
        let route: Route = line.trim().parse().unwrap();
        route
    }));
    *calculate_route_distances(route_tree).iter().max().unwrap()
}

fn build_route_tree(
    routes: impl Iterator<Item = Route>,
) -> HashMap<Location, HashMap<Location, Distance>> {
    routes.fold(
        HashMap::new(),
        |mut routes: HashMap<Location, HashMap<Location, Distance>>, route| {
            routes
                .entry(route.clone().origin)
                .or_default()
                .insert(route.clone().destination, route.clone().distance);

            routes
                .entry(route.destination)
                .or_default()
                .insert(route.origin, route.distance);

            routes
        },
    )
}

fn calculate_route_distances(
    route_tree: HashMap<Location, HashMap<Location, Distance>>,
) -> Vec<usize> {
    route_tree
        .keys()
        .permutations(route_tree.keys().len())
        .map(|route| {
            route
                .into_iter()
                .tuple_windows()
                .map(|(from, to)| *route_tree[from][to])
                .sum()
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Route {
    origin: Location,
    destination: Location,
    distance: Distance,
}

impl FromStr for Route {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use nom::{
            bytes::complete::tag,
            character::complete::{alpha1, digit1},
            combinator::{map, map_res},
            sequence::tuple,
        };

        fn location(input: &str) -> nom::IResult<&str, Location> {
            map(alpha1, |name: &str| Location {
                name: name.to_string(),
            })(input)
        }

        let (_, (origin, _, destination, _, distance)) = tuple((
            location,
            tag(" to "),
            location,
            tag(" = "),
            map_res(digit1, |s: &str| s.parse()),
        ))(string)
        .map_err(|_| string.to_string())?;

        Ok(Self {
            origin,
            destination,
            distance,
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location {
    name: String,
}

#[derive(Clone, Debug)]
struct Distance(usize);

impl FromStr for Distance {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .parse::<usize>()
            .map(Self)
            .map_err(|_| string.to_string())
    }
}

impl Deref for Distance {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(
            part_1(
                "
                London to Dublin = 464
                London to Belfast = 518
                Dublin to Belfast = 141
                "
            ),
            605
        );

        assert_eq!(part_1(INPUT), 251);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(
            part_2(
                "
                London to Dublin = 464
                London to Belfast = 518
                Dublin to Belfast = 141
                "
            ),
            982
        );

        assert_eq!(part_2(INPUT), 898);
    }
}
