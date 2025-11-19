use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, space1},
    combinator::{map, map_res},
};

pub fn parse(input: &str) -> nom::IResult<&str, (usize, usize, char, &str)> {
    map(
        (times, tag("-"), times, space1, anychar, tag(": "), alpha1),
        |(a, _, b, _, letter, _, password)| (a, b, letter, password),
    )
    .parse(input)
}

fn times(input: &str) -> nom::IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse()).parse(input)
}
