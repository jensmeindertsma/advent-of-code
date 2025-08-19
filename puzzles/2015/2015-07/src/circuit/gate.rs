use super::Circuit;

#[derive(Clone, Debug)]
pub enum Gate<'a> {
    Connect { input: Source<'a> },

    And { left: Source<'a>, right: Source<'a> },
    Or { left: Source<'a>, right: Source<'a> },

    Not { input: Source<'a> },

    LeftShift { left: Source<'a>, right: Source<'a> },
    RightShift { left: Source<'a>, right: Source<'a> },
}

impl<'a> Gate<'a> {
    pub fn evaluate(&self, circuit: &mut Circuit<'a>) -> u16 {
        match self {
            Self::Connect { input } => match input {
                Source::Signal(signal) => *signal,
                Source::Wire(wire) => circuit.signal(wire),
            },

            Self::And { left, right } => left.evaluate(circuit) & right.evaluate(circuit),
            Self::Or { left, right } => left.evaluate(circuit) | right.evaluate(circuit),

            Self::Not { input } => !input.evaluate(circuit),

            Self::LeftShift { left, right } => left.evaluate(circuit) << right.evaluate(circuit),
            Self::RightShift { left, right } => left.evaluate(circuit) >> right.evaluate(circuit),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Source<'a> {
    Wire(&'a str),
    Signal(u16),
}

impl<'a> Source<'a> {
    fn evaluate(&self, circuit: &mut Circuit<'a>) -> u16 {
        match self {
            Self::Signal(signal) => *signal,
            Self::Wire(wire) => circuit.signal(wire),
        }
    }
}
