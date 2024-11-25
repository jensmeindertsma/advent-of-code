use crate::common::Instruction;

pub fn part_1(input: &str) -> isize {
    input
        .trim()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap())
        .fold(0, |floor, instruction| match instruction {
            Instruction::Ascend => floor + 1,
            Instruction::Descend => floor - 1,
        })
}
