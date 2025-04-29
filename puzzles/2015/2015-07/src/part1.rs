use crate::{circuit::Circuit, instruction::Instruction};

pub fn part_1(input: &str) -> u16 {
    let mut circuit = Circuit::build(
        input
            .trim()
            .lines()
            .map(|line| Instruction::try_from(line.trim()).unwrap()),
    );

    circuit.signal("a")
}
