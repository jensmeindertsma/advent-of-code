use crate::shared::Cuboid;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
};

pub fn parse(input: &str) -> Cuboid {
    let (_, cuboid) = cuboid(input.trim()).expect("parsing valid input should never fail");

    cuboid
}

fn cuboid(input: &str) -> nom::IResult<&str, Cuboid> {
    fn size(input: &str) -> nom::IResult<&str, usize> {
        map_res(digit1, |x: &str| x.parse()).parse(input)
    }

    map(
        (size, tag("x"), size, tag("x"), size),
        |(length, _, width, _, height)| Cuboid {
            length,
            width,
            height,
        },
    )
    .parse(input)
}
