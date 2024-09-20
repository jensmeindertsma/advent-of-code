use std::collections::HashMap;

use crate::instruction::{Gate, Instruction, Value, Wire};

pub struct Circuit<'a> {
    gates: HashMap<Wire<'a>, Gate<'a>>,
    signals: HashMap<Wire<'a>, u16>,
}

impl<'a> Circuit<'a> {
    pub fn build(instructions: impl Iterator<Item = Instruction<'a>>) -> Self {
        Self {
            gates: instructions.fold(HashMap::new(), |mut gates, instruction| {
                gates.insert(instruction.output, instruction.gate);
                gates
            }),
            signals: HashMap::new(),
        }
    }

    pub fn set_signal(&mut self, wire: impl Into<Wire<'a>>, signal: u16) {
        *self.gates.get_mut(&wire.into()).unwrap() = Gate::Signal(signal)
    }

    pub fn reset(&mut self) {
        self.signals.clear()
    }

    pub fn signal(&mut self, wire: impl Into<Wire<'a>>) -> u16 {
        let wire = wire.into();

        if let Some(signal) = self.signals.get(&wire) {
            return *signal;
        }

        let signal = match self.gates.get(&wire).unwrap().clone() {
            Gate::Signal(signal) => return signal,
            Gate::Connect(identifier) => self.signal(identifier),
            Gate::And { wires: (a, b) } => {
                let a = match a {
                    Value::Signal(signal) => signal,
                    Value::Wire(wire) => self.signal(wire),
                };
                let b = match b {
                    Value::Signal(signal) => signal,
                    Value::Wire(wire) => self.signal(wire),
                };

                a & b
            }
            Gate::Or { wires: (a, b) } => {
                let a = match a {
                    Value::Signal(signal) => signal,
                    Value::Wire(wire) => self.signal(wire),
                };
                let b = match b {
                    Value::Signal(signal) => signal,
                    Value::Wire(wire) => self.signal(wire),
                };

                a | b
            }
            Gate::Not(wire) => {
                let signal = self.signal(wire);

                !signal
            }
            Gate::LeftShift { wire, amount } => {
                let signal = self.signal(wire);

                signal << amount
            }
            Gate::RightShift { wire, amount } => {
                let signal = self.signal(wire);

                signal >> amount
            }
        };

        self.signals
            .entry(wire)
            .and_modify(|e| *e = signal)
            .or_insert(signal);
        signal
    }
}
