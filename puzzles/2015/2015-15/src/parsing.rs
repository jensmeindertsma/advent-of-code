use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
};

pub struct Ingredient<'a> {
    name: &'a str,
    pub capacity: i64,
    pub durability: i64,
    pub flavor: i64,
    pub texture: i64,
    pub calories: u64,
}

pub fn ingredient(input: &str) -> Option<Ingredient<'_>> {
    map(
        (
            alpha1,
            tag(": capacity "),
            property,
            tag(", durability "),
            property,
            tag(", flavor "),
            property,
            tag(", texture "),
            property,
            tag(", calories "),
            calories,
        ),
        |(name, _, capacity, _, durability, _, flavor, _, texture, _, calories)| Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    )
    .parse(input)
    .map(|(_, ingredient)| ingredient)
    .ok()
}

fn property(input: &str) -> nom::IResult<&str, i64> {
    alt((
        map(
            (tag("-"), map_res(digit1, |s: &str| s.parse::<i64>())),
            |(_, n)| -n,
        ),
        map_res(digit1, |s: &str| s.parse()),
    ))
    .parse(input)
}

fn calories(input: &str) -> nom::IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse()).parse(input)
}
