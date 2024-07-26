use crate::Ingredient;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::{map, map_res, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

pub fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(
        tuple((
            name,
            property("capacity"),
            property("durability"),
            property("flavor"),
            property("texture"),
            property("calories"),
        )),
        |(name, capacity, durability, flavor, texture, calories)| Ingredient {
            _name: name,
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

fn property<'a>(
    name: &'a str,
) -> impl FnMut(&'a str) -> std::result::Result<(&'a str, i8), nom::Err<nom::error::Error<&str>>> {
    move |input: &'a str| {
        map(
            tuple((tag(format!("{name} ").as_str()), value, opt(tag(", ")))),
            |(_, value, _)| value,
        )(input)
    }
}

fn value(input: &str) -> IResult<&str, i8> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse()
    })(input)
}
