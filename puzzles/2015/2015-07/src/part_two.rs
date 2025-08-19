use crate::{
    circuit::{
        Circuit,
        gate::{Gate, Source},
    },
    instruction::Instruction,
};

pub fn part_two(input: &str) -> u16 {
    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|line| Instruction::parse(line).unwrap())
        .collect();

    let mut circuit = Circuit::new();

    for instruction in instructions {
        circuit.set(instruction.output_wire, instruction.gate);
    }

    let mut second_circuit = circuit.clone();

    let alpha_signal = circuit.signal("a");

    second_circuit.set(
        "b",
        Gate::Connect {
            input: Source::Signal(alpha_signal),
        },
    );

    second_circuit.signal("a")
}
