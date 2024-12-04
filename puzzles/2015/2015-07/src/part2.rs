use crate::common::{Circuit, Instruction};

pub fn part_2(input: &str) -> u16 {
    let mut circuit = Circuit::build(
        input
            .trim()
            .lines()
            .map(|line| Instruction::try_from(line.trim()).unwrap()),
    );

    let a = circuit.signal("a");

    circuit.set_signal("b", a);

    circuit.reset();

    circuit.signal("a")
}
