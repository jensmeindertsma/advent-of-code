use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    sequence::tuple,
};

pub fn part_1(input: &str) -> isize {
    let relations = input
        .trim()
        .lines()
        .map(|line| relation(line.trim()).unwrap().1)
        .fold(
            HashMap::new(),
            |mut relations: HashMap<String, HashMap<String, isize>>, relation| {
                relations
                    .entry(relation.person)
                    .or_default()
                    .insert(relation.neighbor, relation.delta);
                relations
            },
        );

    let k = relations.keys().len();

    let arrangements = relations.into_iter().permutations(k);

    arrangements
        .map(|mut arrangement| {
            arrangement.push(arrangement.first().unwrap().clone());
            arrangement
                .iter()
                .tuple_windows()
                .map(|((a_name, a_relations), (b_name, b_relations))| {
                    a_relations.get(b_name).unwrap() + b_relations.get(a_name).unwrap()
                })
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> isize {
    let mut relations = input
        .trim()
        .lines()
        .map(|line| relation(line.trim()).unwrap().1)
        .fold(
            HashMap::new(),
            |mut relations: HashMap<String, HashMap<String, isize>>, relation| {
                relations
                    .entry(relation.person)
                    .or_default()
                    .insert(relation.neighbor, relation.delta);
                relations
            },
        );

    relations.insert(
        "Jens".to_string(),
        relations.keys().map(|k| (k.clone(), 0isize)).collect(),
    );

    relations.iter_mut().for_each(|(_, v)| {
        v.insert("Jens".to_string(), 0);
    });

    let k = relations.keys().len();

    let arrangements = relations.into_iter().permutations(k);

    arrangements
        .map(|mut arrangement| {
            arrangement.push(arrangement.first().unwrap().clone());
            arrangement
                .iter()
                .tuple_windows()
                .map(|((a_name, a_relations), (b_name, b_relations))| {
                    a_relations.get(b_name).unwrap() + b_relations.get(a_name).unwrap()
                })
                .sum()
        })
        .max()
        .unwrap()
}

struct Relation {
    person: String,
    neighbor: String,
    delta: isize,
}

fn relation(input: &str) -> nom::IResult<&str, Relation> {
    map(
        tuple((
            alpha1,
            tag(" would "),
            map(
                tuple((
                    alt((
                        tuple((tag("gain "), map_res(digit1, |s: &str| s.parse::<isize>()))),
                        tuple((
                            tag("lose "),
                            map_res(digit1, |s: &str| s.parse::<isize>().map(|i| -i)),
                        )),
                    )),
                    tag(" happiness units"),
                )),
                |((_, delta), _)| delta,
            ),
            tag(" by sitting next to "),
            alpha1,
            tag("."),
        )),
        |(person, _, delta, _, neighbor, _)| Relation {
            person: person.to_string(),
            neighbor: neighbor.to_string(),
            delta,
        },
    )(input)
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
                    Alice would gain 54 happiness units by sitting next to Bob.
                    Alice would lose 79 happiness units by sitting next to Carol.
                    Alice would lose 2 happiness units by sitting next to David.
                    Bob would gain 83 happiness units by sitting next to Alice.
                    Bob would lose 7 happiness units by sitting next to Carol.
                    Bob would lose 63 happiness units by sitting next to David.
                    Carol would lose 62 happiness units by sitting next to Alice.
                    Carol would gain 60 happiness units by sitting next to Bob.
                    Carol would gain 55 happiness units by sitting next to David.
                    David would gain 46 happiness units by sitting next to Alice.
                    David would lose 7 happiness units by sitting next to Bob.
                    David would gain 41 happiness units by sitting next to Carol.
                "
            ),
            330
        );

        assert_eq!(part_1(INPUT), 664);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(INPUT), 640);
    }
}
