use itertools::Itertools;

use crate::parsing;

pub fn part_one(input: &str) -> usize {
    let ingredients: Vec<_> = input
        .trim()
        .lines()
        .map(parsing::ingredient)
        .map(|result| result.expect("each line should have a valid cookie description"))
        .collect();

    ingredients
        .iter()
        .combinations_with_replacement(100)
        .map(|ingredients| {
            let capacity: i64 = ingredients.iter().map(|i| i.capacity).sum();
            let durability: i64 = ingredients.iter().map(|i| i.durability).sum();
            let flavor: i64 = ingredients.iter().map(|i| i.flavor).sum();
            let texture: i64 = ingredients.iter().map(|i| i.texture).sum();

            println!("{capacity} {durability} {flavor} {texture}");

            let score: i64 = capacity * durability * flavor * texture;

            if score < 0 { 0 } else { score as usize }
        })
        .max()
        .expect("there should be a best cookie")
}
