use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    sequence::{delimited, tuple},
};

use crate::instruction::{Coordinate, Instruction, InstructionKind};

pub fn instruction(input: &str) -> nom::IResult<&str, Instruction> {
    map(
        tuple((
            ws(instruction_kind),
            ws(coordinate),
            ws(tag("through")),
            ws(coordinate),
        )),
        |(kind, from, _, to)| Instruction { kind, from, to },
    )(input)
}

fn coordinate(input: &str) -> nom::IResult<&str, Coordinate> {
    map_res(
        tuple((digit1, tag(","), digit1)),
        |(x, _, y): (&str, &str, &str)| -> Result<Coordinate, std::num::ParseIntError> {
            Ok(Coordinate::new(x.parse()?, y.parse()?))
        },
    )(input)
}

fn instruction_kind(input: &str) -> nom::IResult<&str, InstructionKind> {
    alt((
        map(tag("toggle"), |_| InstructionKind::Toggle),
        map(tag("turn on"), |_| InstructionKind::TurnOn),
        map(tag("turn off"), |_| InstructionKind::TurnOff),
    ))(input)
}

// This function allows you to permit whitespace around a parser by wrapping it
// in this function.
fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> nom::IResult<&'a str, O>
where
    F: Fn(&'a str) -> nom::IResult<&'a str, O> + 'a,
{
    delimited(space0, inner, space0)
}

#[cfg(test)]
mod tests {
    use super::{Coordinate, Instruction, InstructionKind};

    #[test]
    fn instruction_parsing() {
        assert_eq!(
            Instruction::try_from("turn on 0,0 through 999,999"),
            Ok(Instruction {
                kind: InstructionKind::TurnOn,
                from: Coordinate::new(0, 0),
                to: Coordinate::new(999, 999)
            })
        );

        assert_eq!(
            Instruction::try_from("toggle 0,0 through 999,0"),
            Ok(Instruction {
                kind: InstructionKind::Toggle,
                from: Coordinate::new(0, 0),
                to: Coordinate::new(999, 0)
            })
        );

        assert_eq!(
            Instruction::try_from("turn off 499,499 through 500,500"),
            Ok(Instruction {
                kind: InstructionKind::TurnOff,
                from: Coordinate::new(499, 499),
                to: Coordinate::new(500, 500)
            })
        );
    }
}
