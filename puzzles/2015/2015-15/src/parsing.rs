use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
};

#[derive(Clone, Copy, Debug)]
pub struct Ingredient {
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: u32,
}

pub fn ingredient(input: &str) -> Option<Ingredient> {
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
        |(_, _, capacity, _, durability, _, flavor, _, texture, _, calories)| Ingredient {
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

fn property(input: &str) -> nom::IResult<&str, i32> {
    alt((
        map(
            (tag("-"), map_res(digit1, |s: &str| s.parse::<i32>())),
            |(_, n)| -n,
        ),
        map_res(digit1, |s: &str| s.parse()),
    ))
    .parse(input)
}

fn calories(input: &str) -> nom::IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse()).parse(input)
}
