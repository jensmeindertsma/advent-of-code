use crate::instruction::{Instruction, parsing};

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parsing::instruction)
        .map(|result| result.expect("every line should be a valid instruction"))
        .fold(vec![[Light::Off; 1000]; 1000], |mut grid, instruction| {
            match instruction {
                Instruction::TurnOn { from, to } => {
                    for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                        for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                            *light = Light::On
                        }
                    }
                }
                Instruction::TurnOff { from, to } => {
                    for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                        for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                            *light = Light::Off
                        }
                    }
                }
                Instruction::Toggle { from, to } => {
                    for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                        for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                            let current = *light;

                            *light = match current {
                                Light::On => Light::Off,
                                Light::Off => Light::On,
                            }
                        }
                    }
                }
            };

            grid
        })
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&light| light == Light::On)
        .count()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Light {
    On,
    Off,
}
