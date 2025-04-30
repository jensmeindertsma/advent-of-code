use crate::{
    grid::Grid,
    instruction::{Instruction, Task},
};

pub fn part_1(input: &str) -> usize {
    let mut grid = Grid::new(1000, Light::Off);

    grid.process(
        input
            .trim()
            .lines()
            .map(|line| Instruction::parse(line).unwrap()),
        |task, light| match task {
            Task::Toggle => {
                *light = match light {
                    Light::On => Light::Off,
                    Light::Off => Light::On,
                }
            }
            Task::Enable => *light = Light::On,
            Task::Disable => *light = Light::Off,
        },
    );

    grid.columns()
        .map(|column| column.iter().filter(|light| **light == Light::On).count())
        .sum()
}

#[derive(Clone, Copy, PartialEq)]
enum Light {
    On,
    Off,
}
