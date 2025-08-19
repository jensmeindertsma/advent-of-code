use super::circuit::gate::{Gate, Source};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u16},
    combinator::map,
};

#[derive(Debug)]
pub struct Instruction<'a> {
    pub gate: Gate<'a>,
    pub output_wire: &'a str,
}

impl<'a> Instruction<'a> {
    pub fn parse(input: &'a str) -> Result<Self, ()> {
        fn source(input: &str) -> nom::IResult<&str, Source<'_>> {
            alt((map(alpha1, Source::Wire), map(u16, Source::Signal))).parse(input)
        }

        fn and(input: &'_ str) -> nom::IResult<&'_ str, Gate<'_>> {
            map((source, tag(" AND "), source), |(left, _, right)| {
                Gate::And { left, right }
            })
            .parse(input)
        }

        fn or(input: &'_ str) -> nom::IResult<&'_ str, Gate<'_>> {
            map((source, tag(" OR "), source), |(left, _, right)| Gate::Or {
                left,
                right,
            })
            .parse(input)
        }

        fn not(input: &'_ str) -> nom::IResult<&'_ str, Gate<'_>> {
            map((tag("NOT "), source), |(_, input)| Gate::Not { input }).parse(input)
        }

        fn left_shift(input: &'_ str) -> nom::IResult<&'_ str, Gate<'_>> {
            map((source, tag(" LSHIFT "), source), |(left, _, right)| {
                Gate::LeftShift { left, right }
            })
            .parse(input)
        }

        fn right_shift(input: &'_ str) -> nom::IResult<&'_ str, Gate<'_>> {
            map((source, tag(" RSHIFT "), source), |(left, _, right)| {
                Gate::RightShift { left, right }
            })
            .parse(input)
        }

        fn connect(input: &'_ str) -> nom::IResult<&'_ str, Gate<'_>> {
            map(source, |input| Gate::Connect { input }).parse(input)
        }

        Ok(map(
            (
                alt((and, or, not, left_shift, right_shift, connect)),
                tag(" -> "),
                alpha1,
            ),
            |(gate, _, output_wire)| Self { gate, output_wire },
        )
        .parse(input)
        .map_err(|_| ())?
        .1)
    }
}
