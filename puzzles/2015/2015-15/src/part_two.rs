use crate::parsing;
use itertools::Itertools;

pub fn part_two(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(parsing::ingredient)
        .map(|result| result.expect("each line should have a valid cookie description"))
        .combinations_with_replacement(100)
        .filter_map(|ingredients| {
            let capacity: u32 = ingredients
                .iter()
                .map(|ingredient| ingredient.capacity)
                .sum::<i32>()
                .max(0) as u32;

            let durability: u32 = ingredients
                .iter()
                .map(|ingredient| ingredient.durability)
                .sum::<i32>()
                .max(0) as u32;

            let flavor: u32 = ingredients
                .iter()
                .map(|ingredient| ingredient.flavor)
                .sum::<i32>()
                .max(0) as u32;

            let texture: u32 = ingredients
                .iter()
                .map(|ingredient| ingredient.texture)
                .sum::<i32>()
                .max(0) as u32;

            let calories: u32 = ingredients
                .iter()
                .map(|ingredient| ingredient.calories)
                .sum();

            if calories == 500 {
                Some(capacity * durability * flavor * texture)
            } else {
                None
            }
        })
        .max()
        .expect("there should be a winning cookie")
}
