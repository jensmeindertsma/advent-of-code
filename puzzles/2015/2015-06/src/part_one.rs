use crate::parsing::{self, Instruction, Operation};

pub fn part_one(input: &str) -> usize {
    #[derive(Clone, Copy, PartialEq)]
    enum Light {
        On,
        Off,
    }

    input
        .trim()
        .lines()
        .fold([[Light::Off; 1000]; 1000], |mut grid, line| {
            let Instruction { from, to, kind } = parsing::parse(line.trim());

            for row in &mut grid[from.y..=to.y] {
                for cell in &mut row[from.x..=to.x] {
                    match kind {
                        Operation::Toggle => {
                            if *cell == Light::On {
                                *cell = Light::Off
                            } else {
                                *cell = Light::On
                            }
                        }
                        Operation::TurnOff => *cell = Light::Off,
                        Operation::TurnOn => *cell = Light::On,
                    }
                }
            }

            grid
        })
        .map(|row| row.iter().filter(|l| **l == Light::On).count())
        .iter()
        .sum()
}
