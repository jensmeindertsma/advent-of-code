use crate::common::Instruction;

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap())
        .scan(0, |floor, instruction| {
            match instruction {
                Instruction::Ascend => *floor += 1,
                Instruction::Descend => *floor -= 1,
            }
            Some(*floor)
        })
        .enumerate()
        .find(|&(_, floor): &(_, i32)| floor.is_negative())
        .map(|(index, _)| index + 1)
        .unwrap()
}
