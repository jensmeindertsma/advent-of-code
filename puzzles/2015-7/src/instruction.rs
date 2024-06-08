#[derive(Clone, Debug, PartialEq)]
pub struct Instruction<'a> {
    pub gate: Gate<'a>,
    pub output: Wire<'a>,
}

impl<'a> TryFrom<&'a str> for Instruction<'a> {
    type Error = &'a str;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        parsing::instruction(string)
            .map(|(_, instruction)| instruction)
            .map_err(|_| string)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Gate<'a> {
    And { wires: (Value<'a>, Value<'a>) },
    Connect(Wire<'a>),
    LeftShift { wire: Wire<'a>, amount: u16 },
    Not(Wire<'a>),
    Or { wires: (Value<'a>, Value<'a>) },
    RightShift { wire: Wire<'a>, amount: u16 },
    Signal(u16),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Wire<'a>(&'a str);

impl<'a> From<&'a str> for Wire<'a> {
    fn from(identifier: &'a str) -> Self {
        Self(identifier)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    Wire(Wire<'a>),
    Signal(u16),
}

impl<'a> From<Wire<'a>> for Value<'a> {
    fn from(wire: Wire<'a>) -> Self {
        Self::Wire(wire)
    }
}

impl<'a> From<u16> for Value<'a> {
    fn from(signal: u16) -> Self {
        Self::Signal(signal)
    }
}

mod parsing {
    use super::{Gate, Instruction, Value, Wire};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::space0,
        combinator::{map, map_res},
        sequence::{delimited, tuple},
    };

    pub fn instruction(input: &str) -> nom::IResult<&str, Instruction> {
        map(tuple((gate, tag("->"), wire)), |(gate, _, output)| {
            Instruction { gate, output }
        })(input)
    }

    fn gate(input: &str) -> nom::IResult<&str, Gate> {
        fn and(input: &str) -> nom::IResult<&str, Gate> {
            map(tuple((value, tag("AND"), value)), |(a, _, b)| Gate::And {
                wires: (a, b),
            })(input)
        }

        fn connect(input: &str) -> nom::IResult<&str, Gate> {
            map(wire, Gate::Connect)(input)
        }

        fn left_shift(input: &str) -> nom::IResult<&str, Gate> {
            map(tuple((wire, tag("LSHIFT"), number)), |(wire, _, amount)| {
                Gate::LeftShift { wire, amount }
            })(input)
        }

        fn or(input: &str) -> nom::IResult<&str, Gate> {
            map(tuple((value, tag("OR"), value)), |(a, _, b)| Gate::Or {
                wires: (a, b),
            })(input)
        }

        fn not(input: &str) -> nom::IResult<&str, Gate> {
            map(tuple((tag("NOT"), wire)), |(_, wire)| Gate::Not(wire))(input)
        }

        fn right_shift(input: &str) -> nom::IResult<&str, Gate> {
            map(tuple((wire, tag("RSHIFT"), number)), |(wire, _, amount)| {
                Gate::RightShift { wire, amount }
            })(input)
        }

        fn signal(input: &str) -> nom::IResult<&str, Gate> {
            map(number, Gate::Signal)(input)
        }

        // `connect` must be last or it will always match other gates too
        // before their respective parsers got a chances.
        alt((and, or, not, left_shift, right_shift, signal, connect))(input)
    }

    fn value(input: &str) -> nom::IResult<&str, Value<'_>> {
        alt((map(wire, Value::Wire), map(number, Value::Signal)))(input)
    }

    fn wire(input: &str) -> nom::IResult<&str, Wire<'_>> {
        map(
            delimited(space0, take_while1(char::is_alphabetic), space0),
            Wire,
        )(input)
    }

    fn number(input: &str) -> nom::IResult<&str, u16> {
        map_res(
            delimited(space0, take_while1(|c: char| c.is_ascii_digit()), space0),
            |number: &str| number.parse(),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{Gate, Instruction, Wire};

    #[test]
    fn and() {
        assert_eq!(
            "x AND y -> z".try_into(),
            Ok(Instruction {
                gate: Gate::And {
                    wires: (Wire::from("x").into(), Wire::from("y").into())
                },
                output: Wire::from("z")
            })
        );
    }

    #[test]
    fn connect() {
        assert_eq!(
            "lb -> fg".try_into(),
            Ok(Instruction {
                gate: Gate::Connect(Wire::from("lb")),
                output: Wire::from("fg")
            })
        );
    }

    #[test]
    fn left_shift() {
        assert_eq!(
            "p LSHIFT 2 -> q".try_into(),
            Ok(Instruction {
                gate: Gate::LeftShift {
                    wire: Wire::from("p"),
                    amount: 2
                },
                output: Wire::from("q")
            })
        );
    }

    #[test]
    fn not() {
        assert_eq!(
            "NOT e -> f".try_into(),
            Ok(Instruction {
                gate: Gate::Not(Wire::from("e")),
                output: Wire::from("f")
            })
        );
    }

    #[test]
    fn or() {
        assert_eq!(
            "x OR y -> e".try_into(),
            Ok(Instruction {
                gate: Gate::Or {
                    wires: (Wire::from("x").into(), Wire::from("y").into())
                },
                output: Wire::from("e")
            })
        );
    }

    #[test]
    fn right_shift() {
        assert_eq!(
            "r RSHIFT 45 -> s".try_into(),
            Ok(Instruction {
                gate: Gate::RightShift {
                    wire: Wire::from("r"),
                    amount: 45
                },
                output: Wire::from("s")
            })
        );
    }

    #[test]
    fn signal() {
        assert_eq!(
            "123 -> x".try_into(),
            Ok(Instruction {
                gate: Gate::Signal(123),
                output: Wire::from("x")
            })
        );
    }
}
