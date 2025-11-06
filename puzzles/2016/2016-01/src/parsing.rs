use crate::instruction::Instruction;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
};

pub fn document(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    separated_list1(tag(", "), instruction).parse(input)
}

fn instruction(input: &str) -> nom::IResult<&str, Instruction> {
    map(
        (
            alt((tag("L"), tag("R"))),
            map_res(digit1, |n: &str| n.parse::<usize>()),
        ),
        |(turn, distance)| match turn {
            "L" => Instruction::Left(distance),
            "R" => Instruction::Right(distance),
            _ => unreachable!(),
        },
    )
    .parse(input)
}
