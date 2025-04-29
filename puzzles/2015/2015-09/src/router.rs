use std::{collections::HashMap, ops::Deref, str::FromStr};

use itertools::Itertools;
use nom::Parser;

pub struct Router {
    network: HashMap<Location, HashMap<Location, Distance>>,
}

impl Router {
    pub fn build_tree(routes: impl Iterator<Item = Route>) -> Self {
        let network = routes.fold(
            HashMap::new(),
            |mut network: HashMap<Location, HashMap<Location, Distance>>, route| {
                network
                    .entry(route.clone().origin)
                    .or_default()
                    .insert(route.clone().destination, route.clone().distance);

                network
                    .entry(route.destination)
                    .or_default()
                    .insert(route.origin, route.distance);

                network
            },
        );

        Self { network }
    }

    pub fn paths(&self) -> Vec<usize> {
        self.network
            .keys()
            .permutations(self.network.keys().len())
            .map(|destinations| {
                destinations
                    .into_iter()
                    .tuple_windows()
                    .map(|(from, to)| *self.network[from][to])
                    .sum()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Route {
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
        };

        fn location(input: &str) -> nom::IResult<&str, Location> {
            map(alpha1, |name: &str| Location {
                name: name.to_string(),
            })
            .parse(input)
        }

        let (_, (origin, _, destination, _, distance)) = (
            location,
            tag(" to "),
            location,
            tag(" = "),
            map_res(digit1, |s: &str| s.parse()),
        )
            .parse(string)
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
