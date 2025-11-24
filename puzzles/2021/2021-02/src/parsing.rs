use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
};

pub fn parse(input: &str) -> Result<Instruction, ()> {
    instruction(input).map(|result| result.1).map_err(|_| ())
}

pub enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

pub fn instruction(input: &str) -> nom::IResult<&str, Instruction> {
    map_res(
        (
            alt((tag("forward"), tag("up"), tag("down"))),
            space1,
            map_res(digit1, |x: &str| x.parse()),
        ),
        |(direction, _, amount)| match direction {
            "forward" => Ok(Instruction::Forward(amount)),
            "up" => Ok(Instruction::Up(amount)),
            "down" => Ok(Instruction::Down(amount)),
            _ => Err("unknown direction"),
        },
    )
    .parse(input)
}
