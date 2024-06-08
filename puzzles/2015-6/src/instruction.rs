use crate::parsing;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub from: Coordinate,
    pub to: Coordinate,
}

impl<'a> TryFrom<&'a str> for Instruction {
    type Error = &'a str;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        parsing::instruction(string)
            .map(|(_, instruction)| instruction)
            .map_err(|_| string)
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    Toggle,
    TurnOff,
    TurnOn,
}

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
