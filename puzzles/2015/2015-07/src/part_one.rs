use crate::{
    circuit::{Circuit, Wire},
    instruction::{Instruction, parsing},
};

pub fn part_one(input: &str) -> u16 {
    let instructions: Vec<Instruction<'_>> = input
        .lines()
        .map(parsing::instruction)
        .map(|result| result.expect("every instruction should be valid"))
        .collect();

    let mut circuit = Circuit::new();

    for instruction in instructions {
        circuit.assemble(instruction);
    }

    circuit.get_signal(Wire("a"))
}
