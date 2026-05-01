use nom::{
    Finish, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
};

pub fn description(input: &str) -> Option<(&str, &str, u16)> {
    map(
        (destination, tag(" to "), destination, tag(" = "), distance),
        |(from, _, to, _, distance)| (from, to, distance),
    )
    .parse(input)
    .finish()
    .ok()
    .map(|(_, result)| result)
}

fn destination(input: &str) -> nom::IResult<&str, &str> {
    alpha1.parse(input)
}

fn distance(input: &str) -> nom::IResult<&str, u16> {
    map_res(digit1, str::parse).parse(input)
}
