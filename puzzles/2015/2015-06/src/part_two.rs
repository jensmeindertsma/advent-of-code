use crate::instruction::{Instruction, parsing};

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parsing::instruction)
        .map(|result| result.expect("every line should be a valid instruction"))
        .fold(
            vec![[Light { brightness: 0 }; 1000]; 1000],
            |mut grid, instruction| {
                match instruction {
                    Instruction::TurnOn { from, to } => {
                        for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                            for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                                light.brightness += 1;
                            }
                        }
                    }
                    Instruction::TurnOff { from, to } => {
                        for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                            for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                                light.brightness = light.brightness.saturating_sub(1);
                            }
                        }
                    }
                    Instruction::Toggle { from, to } => {
                        for row in grid.iter_mut().skip(from.y).take(to.y - from.y + 1) {
                            for light in row.iter_mut().skip(from.x).take(to.x - from.x + 1) {
                                light.brightness += 2;
                            }
                        }
                    }
                };

                grid
            },
        )
        .iter()
        .flat_map(|row| row.iter())
        .map(|&light| light.brightness)
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Light {
    brightness: usize,
}
