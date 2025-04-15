use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    Parser,
};

pub struct Instruction {
    pub kind: InstructionKind,
    pub from: Location,
    pub to: Location,
}

pub enum InstructionKind {
    Disable,
    Enable,
    Toggle,
}

pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        instruction(string)
            .map(|(_, result)| result)
            .map_err(|error| error.to_string())
    }
}

fn instruction(input: &str) -> nom::IResult<&str, Instruction> {
    map(
        (
            alt((
                map(tag("turn on"), |_| InstructionKind::Enable),
                map(tag("turn off"), |_| InstructionKind::Disable),
                map(tag("toggle"), |_| InstructionKind::Toggle),
            )),
            tag(" "),
            location,
            tag(" through "),
            location,
        ),
        |(kind, _, from, _, to)| Instruction { kind, from, to },
    )
    .parse(input)
}

fn location(input: &str) -> nom::IResult<&str, Location> {
    map(
        (
            map_res(digit1, |s: &str| s.parse()),
            tag(","),
            map_res(digit1, |s: &str| s.parse()),
        ),
        |(x, _, y)| Location { x, y },
    )
    .parse(input)
}
