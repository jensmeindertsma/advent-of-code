use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::usize,
    combinator::map_res,
};

pub fn parse(line: &str) -> Instruction {
    fn operation(input: &str) -> IResult<&str, Operation> {
        map_res(
            alt((tag("turn on"), tag("turn off"), tag("toggle"))),
            |string| match string {
                "turn on" => Ok(Operation::TurnOn),
                "turn off" => Ok(Operation::TurnOff),
                "toggle" => Ok(Operation::Toggle),
                _ => Err("unknown operation"),
            },
        )
        .parse(input)
    }

    fn location(input: &str) -> IResult<&str, Location> {
        (usize, tag(","), usize)
            .map(|(x, _, y)| Location { x, y })
            .parse(input)
    }

    let (_, (kind, _, from, _, to)) = (operation, tag(" "), location, tag(" through "), location)
        .parse(line)
        .unwrap();

    Instruction { from, to, kind }
}

pub struct Instruction {
    pub from: Location,
    pub to: Location,
    pub kind: Operation,
}

pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub enum Operation {
    Toggle,
    TurnOff,
    TurnOn,
}
