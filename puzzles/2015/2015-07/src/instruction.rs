use crate::circuit::{Gate, Wire};

#[derive(Clone, Copy, Debug)]
pub struct Instruction<'a> {
    pub gate: Gate<'a>,
    pub output: Wire<'a>,
}

pub mod parsing {
    use crate::{
        circuit::Source,
        instruction::{Gate, Instruction, Wire},
    };
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::{map, map_res},
    };

    pub fn instruction(input: &str) -> Option<Instruction<'_>> {
        (gate, tag(" -> "), wire)
            .parse(input)
            .map(|(_, (gate, _, output))| Instruction { gate, output })
            .ok()
    }

    fn gate(input: &str) -> IResult<&str, Gate<'_>> {
        alt((
            map((source, tag(" LSHIFT "), value), |(source, _, amount)| {
                Gate::LeftShift { source, amount }
            }),
            map((source, tag(" RSHIFT "), value), |(source, _, amount)| {
                Gate::RightShift { source, amount }
            }),
            map((source, tag(" AND "), source), |(left, _, right)| {
                Gate::And { left, right }
            }),
            map((source, tag(" OR "), source), |(left, _, right)| Gate::Or {
                left,
                right,
            }),
            map((tag("NOT "), source), |(_, source)| Gate::Not { source }),
            // Must be last or it will short-circuit
            map(source, Gate::Connect),
        ))
        .parse(input)
    }

    fn source(input: &str) -> IResult<&str, Source<'_>> {
        alt((map(value, Source::Value), map(wire, Source::Wire))).parse(input)
    }

    fn value(input: &str) -> IResult<&str, u16> {
        map_res(digit1, |s: &str| s.parse()).parse(input)
    }

    fn wire(input: &str) -> IResult<&str, Wire<'_>> {
        map(alpha1, Wire).parse(input)
    }
}
