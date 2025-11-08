use crate::shared::Turn;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{iterator, map, map_res, opt},
};

pub fn parse(input: &str) -> impl Iterator<Item = (Turn, usize)> {
    iterator(input, instruction)
}

fn instruction(input: &str) -> nom::IResult<&str, (Turn, usize)> {
    map((turn, distance, opt(tag(", "))), |(turn, distance, _)| {
        (turn, distance)
    })
    .parse(input)
}

fn turn(input: &str) -> nom::IResult<&str, Turn> {
    alt((
        map(tag("L"), |_| Turn::Left),
        map(tag("R"), |_| Turn::Right),
    ))
    .parse(input)
}

fn distance(input: &str) -> nom::IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse()).parse(input)
}
