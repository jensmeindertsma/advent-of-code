use crate::computer::Computer;

pub fn part_one(input: &str) -> usize {
    let mut computer = Computer::with_program(input);

    computer.set(1, 12);
    computer.set(2, 2);

    computer.run().state[0]
}
