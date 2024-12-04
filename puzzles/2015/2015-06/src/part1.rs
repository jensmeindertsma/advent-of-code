use crate::common::{Instruction, InstructionKind};

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<Instruction>().unwrap())
        .fold(vec![[Light::Off; 1000]; 1000], |mut grid, instruction| {
            for column in grid
                .iter_mut()
                .take(instruction.to.x + 1)
                .skip(instruction.from.x)
            {
                for light in column
                    .iter_mut()
                    .take(instruction.to.y + 1)
                    .skip(instruction.from.y)
                {
                    match instruction.kind {
                        InstructionKind::Toggle => {
                            *light = match light {
                                Light::On => Light::Off,
                                Light::Off => Light::On,
                            }
                        }
                        InstructionKind::Enable => *light = Light::On,
                        InstructionKind::Disable => *light = Light::Off,
                    }
                }
            }

            grid
        })
        .iter()
        .map(|column| column.iter().filter(|light| **light == Light::On).count())
        .sum()
}

#[derive(Clone, Copy, PartialEq)]
enum Light {
    On,
    Off,
}
