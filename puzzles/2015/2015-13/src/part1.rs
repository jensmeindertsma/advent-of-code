use crate::common::parse_relation;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_1(input: &str) -> i16 {
    let relations = input
        .trim()
        .lines()
        .map(|line| parse_relation(line.trim()).unwrap().1)
        .fold(
            HashMap::new(),
            |mut relations: HashMap<String, HashMap<String, i16>>, relation| {
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
