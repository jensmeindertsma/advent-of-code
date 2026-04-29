pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parsing::instruction)
        .map(|result| result.expect("every line should be a valid instruction"))
        .fold(vec![[Light::Off; 1000]; 1000], |mut grid, instruction| {
            match instruction {
                Instruction::TurnOn { from, to } => {
                    for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                        for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                            *light = Light::On
                        }
                    }
                }
                Instruction::TurnOff { from, to } => {
                    for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                        for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                            *light = Light::Off
                        }
                    }
                }
                Instruction::Toggle { from, to } => {
                    for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                        for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                            let current = *light;

                            *light = match current {
                                Light::On => Light::Off,
                                Light::Off => Light::On,
                            }
                        }
                    }
                }
            };

            grid
        })
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell == Light::On)
        .count()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Light {
    On,
    Off,
}

enum Instruction {
    TurnOn { from: Coordinate, to: Coordinate },
    TurnOff { from: Coordinate, to: Coordinate },
    Toggle { from: Coordinate, to: Coordinate },
}

struct Coordinate {
    x: usize,
    y: usize,
}

mod parsing {
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        sequence::separated_pair,
    };

    use super::{Coordinate, Instruction};

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
