use crate::Ingredient;
use nom::{
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::tuple,
    IResult,
};

pub fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(
        tuple((name, capacity, durability, flavor, texture, calories)),
        |(name, capacity, durability, flavor, texture, calories)| Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    )(input)
}

fn name(input: &str) -> IResult<&str, String> {
    map(
        tuple((map(take_until(": "), |s: &str| s.to_owned()), tag(": "))),
        |(s, _)| s,
    )(input)
}
