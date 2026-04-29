use crate::{
    circuit::{Circuit, Gate, Source, Wire},
    instruction::{Instruction, parsing},
};

pub fn part_two(input: &str) -> u16 {
    let instructions: Vec<Instruction<'_>> = input
        .lines()
        .map(parsing::instruction)
        .map(|result| result.expect("every instruction should be valid"))
        .collect();

    let mut circuit = Circuit::new();

    for instruction in instructions {
        circuit.assemble(instruction);
    }

    let mut second_circuit = circuit.clone();

    let signal = circuit.get_signal(Wire("a"));

    second_circuit.set_signal(Wire("b"), Gate::Connect(Source::Value(signal)));

    second_circuit.get_signal(Wire("a"))
}
