pub enum Instruction {
    TurnOn { from: Coordinate, to: Coordinate },
    TurnOff { from: Coordinate, to: Coordinate },
    Toggle { from: Coordinate, to: Coordinate },
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

pub mod parsing {
    use super::{Coordinate, Instruction};
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        sequence::separated_pair,
    };

    pub fn instruction(input: &str) -> Option<Instruction> {
        (kind, coordinate, tag(" through "), coordinate)
            .parse(input)
            .map(|(_, (kind, from, _, to))| match kind {
                Kind::On => Instruction::TurnOn { from, to },
                Kind::Off => Instruction::TurnOff { from, to },
                Kind::Toggle => Instruction::Toggle { from, to },
            })
            .ok()
    }

    enum Kind {
        On,
        Off,
        Toggle,
    }

    fn kind(input: &str) -> IResult<&str, Kind> {
        alt((
            map(tag("turn on "), |_| Kind::On),
            map(tag("turn off "), |_| Kind::Off),
            map(tag("toggle "), |_| Kind::Toggle),
        ))
        .parse(input)
    }

    fn coordinate(input: &str) -> IResult<&str, Coordinate> {
        map(
            separated_pair(
                map_res(digit1, |s: &str| s.parse()),
                tag(","),
                map_res(digit1, |s: &str| s.parse()),
            ),
            |(x, y)| Coordinate { x, y },
        )
        .parse(input)
    }
}
