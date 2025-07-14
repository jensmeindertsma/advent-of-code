use crate::present::Present;
use nom::{Parser, bytes::complete as bytes, character::complete as character, combinator};

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            fn dimension(input: &str) -> nom::IResult<&str, usize> {
                combinator::map_res(character::digit1, str::parse).parse(input)
            }

            let (_, (length, _, width, _, height)) = (
                dimension,
                bytes::tag("x"),
                dimension,
                bytes::tag("x"),
                dimension,
            )
                .parse(line)
                .unwrap();

            Present::new(length, width, height)
        })
        .map(|present| {
            let sides = present.sides();

            let surface_area: usize = sides.iter().map(|side| side.area()).sum();
            let extra: usize = sides.iter().map(|side| side.area()).min().unwrap();

            surface_area + extra
        })
        .sum()
}
