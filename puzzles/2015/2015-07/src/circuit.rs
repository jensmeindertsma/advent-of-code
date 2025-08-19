pub mod gate;

use gate::{Gate, Source};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Circuit<'a> {
    parts: HashMap<&'a str, Gate<'a>>,
}

impl<'a> Circuit<'a> {
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
        }
    }

    pub fn set(&mut self, wire: &'a str, gate: Gate<'a>) {
        self.parts.insert(wire, gate);
    }

    pub fn signal(&mut self, wire: &'a str) -> u16 {
        let gate = self
            .parts
            .remove(wire)
            .expect("wire identifier  should be known in map");

        let signal = gate.evaluate(self);

        // `HashMap::insert` replaces the existing value
        self.parts.insert(
            wire,
            Gate::Connect {
                input: Source::Signal(signal),
            },
        );

        signal
    }
}
