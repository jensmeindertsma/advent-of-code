mod parsing;

use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse::<Ingredient>().unwrap())
        .combinations_with_replacement(100)
        .enumerate()
        .for_each(|(n, recipe)| {
            let mut ingredients = HashMap::new();

            for ingredient in recipe {
                ingredients
                    .entry(ingredient.name)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }

            println!("--- Recipe {n} ---");
            for (name, count) in ingredients {
                println!("{name} = {count}")
            }
        });

    todo!()
}

#[derive(Clone)]
struct Ingredient {
    name: String,
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: i8,
}

impl FromStr for Ingredient {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parsing::ingredient(string.trim())
            .map(|(_, ingredient)| ingredient)
            .map_err(|_| string.to_owned())
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    const EXAMPLE_INPUT: &str = "
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    ";

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(EXAMPLE_INPUT), 62842880);

        // assert_eq!(part_1(INPUT), ???);
    }
}
