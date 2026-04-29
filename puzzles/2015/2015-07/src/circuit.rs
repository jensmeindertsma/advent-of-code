use crate::instruction::Instruction;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Circuit<'a> {
    parts: HashMap<Wire<'a>, Gate<'a>>,
}

impl<'a> Circuit<'a> {
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
        }
    }

    pub fn assemble(&mut self, instruction: Instruction<'a>) {
        self.parts.insert(instruction.output, instruction.gate);
    }

    pub fn get_signal(&mut self, wire: Wire<'a>) -> u16 {
        let gate = self
            .parts
            .remove(&wire)
            .expect("wire identifier  should be known in map");

        let signal = gate.evaluate(self);

        self.parts
            .insert(wire, Gate::Connect(Source::Value(signal)));

        signal
    }

    pub fn set_signal(&mut self, wire: Wire<'a>, gate: Gate<'a>) {
        self.parts.insert(wire, gate);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Wire<'a>(pub &'a str);

#[derive(Clone, Copy, Debug)]
pub enum Gate<'a> {
    Connect(Source<'a>),

    And { left: Source<'a>, right: Source<'a> },

    Or { left: Source<'a>, right: Source<'a> },

    Not { source: Source<'a> },

    LeftShift { source: Source<'a>, amount: u16 },

    RightShift { source: Source<'a>, amount: u16 },
}

impl<'a> Gate<'a> {
    pub fn evaluate(&self, circuit: &mut Circuit<'a>) -> u16 {
        match self {
            Self::Connect(source) => source.evaluate(circuit),

            Self::And { left, right } => left.evaluate(circuit) & right.evaluate(circuit),
            Self::Or { left, right } => left.evaluate(circuit) | right.evaluate(circuit),

            Self::Not { source } => !source.evaluate(circuit),

            Self::LeftShift { source, amount } => source.evaluate(circuit) << amount,
            Self::RightShift { source, amount } => source.evaluate(circuit) >> amount,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Source<'a> {
    Value(u16),
    Wire(Wire<'a>),
}

impl<'a> Source<'a> {
    fn evaluate(&self, circuit: &mut Circuit<'a>) -> u16 {
        match self {
            Self::Value(signal) => *signal,
            Self::Wire(wire) => circuit.get_signal(*wire),
        }
    }
}
