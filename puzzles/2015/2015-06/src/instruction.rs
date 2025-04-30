use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
};
use ornament::ParseError;

pub struct Instruction {
    pub task: Task,
    pub from: Location,
    pub to: Location,
}

pub enum Task {
    Disable,
    Enable,
    Toggle,
}

pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Instruction {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        map(
            (
                alt((
                    map(tag("turn on"), |_| Task::Enable),
                    map(tag("turn off"), |_| Task::Disable),
                    map(tag("toggle"), |_| Task::Toggle),
                )),
                tag(" "),
                location,
                tag(" through "),
                location,
            ),
            |(task, _, from, _, to)| Instruction { task, from, to },
        )
        .parse(input.trim())
        .map(|(_, instruction)| instruction)
    }
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
