use crate::{circuit::Circuit, instruction::Instruction};

pub fn part_one(input: &str) -> u16 {
    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|line| Instruction::parse(line).unwrap())
        .collect();

    let mut circuit = Circuit::new();

    for instruction in instructions {
        circuit.set(instruction.output_wire, instruction.gate);
    }

    circuit.signal("a")
}
