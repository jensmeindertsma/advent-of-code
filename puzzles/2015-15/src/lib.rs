mod parsing;

use itertools::Itertools;
use std::str::FromStr;

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse::<Ingredient>().unwrap())
        .combinations_with_replacement(100)
        .map(|recipe| {
            recipe
                .iter()
                .fold(Cookie::default(), |mut cookie, ingredient| {
                    cookie.capacity += ingredient.capacity as isize;
                    cookie.durability += ingredient.durability as isize;
                    cookie.flavor += ingredient.flavor as isize;
                    cookie.texture += ingredient.texture as isize;

                    cookie
                })
        })
        .map(calculate_score)
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse::<Ingredient>().unwrap())
        .combinations_with_replacement(100)
        .map(|recipe| {
            recipe
                .iter()
                .fold(Cookie::default(), |mut cookie, ingredient| {
                    cookie.capacity += ingredient.capacity as isize;
                    cookie.durability += ingredient.durability as isize;
                    cookie.flavor += ingredient.flavor as isize;
                    cookie.texture += ingredient.texture as isize;
                    cookie.calories += ingredient.calories as isize;

                    cookie
                })
        })
        .filter(|cookie| cookie.calories == 500)
        .map(calculate_score)
        .max()
        .unwrap()
}

#[derive(Clone)]
struct Ingredient {
    _name: String,
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

#[derive(Default)]
struct Cookie {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn calculate_score(cookie: Cookie) -> usize {
    let capacity = if cookie.capacity.is_negative() {
        0
    } else {
        cookie.capacity.try_into().unwrap()
    };

    let durability = if cookie.durability.is_negative() {
        0
    } else {
        cookie.durability.try_into().unwrap()
    };

    let flavor = if cookie.flavor.is_negative() {
        0
    } else {
        cookie.flavor.try_into().unwrap()
    };

    let texture = if cookie.texture.is_negative() {
        0
    } else {
        cookie.texture.try_into().unwrap()
    };

    capacity * durability * flavor * texture
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

        assert_eq!(part_1(INPUT), 21367368);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(EXAMPLE_INPUT), 57600000);

        assert_eq!(part_2(INPUT), 1766400);
    }
}
