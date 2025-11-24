use crate::parsing::{self, Instruction};

pub fn part_two(input: &str) -> isize {
    let mut position: isize = 0;
    let mut aim: isize = 0;
    let mut depth: isize = 0;

    for instruction in input
        .trim()
        .lines()
        .map(|line| parsing::parse(line).unwrap())
    {
        match instruction {
            Instruction::Up(amount) => aim += amount as isize,
            Instruction::Down(amount) => aim -= amount as isize,
            Instruction::Forward(amount) => {
                position += amount as isize;
                depth += aim * amount as isize
            }
        };
    }

    (position * depth).abs()
}
