use crate::parsing::{self, Instruction};

pub fn part_one(input: &str) -> usize {
    let mut position: isize = 0;
    let mut depth: isize = 0;

    for instruction in input
        .trim()
        .lines()
        .map(|line| parsing::parse(line).unwrap())
    {
        match instruction {
            Instruction::Forward(amount) => {
                position += amount as isize;
            }
            Instruction::Up(amount) => {
                depth -= amount as isize;
            }
            Instruction::Down(amount) => {
                depth += amount as isize;
            }
        };
    }

    (position * depth).try_into().unwrap()
}
