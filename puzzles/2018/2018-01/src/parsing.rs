use crate::shared::Change;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
};

pub fn parse(input: &str) -> Change {
    let (_, change) = (char, amount).unwrap();
    change
}

fn amount(input: &str) -> nom::IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse()).parse(input)
}
