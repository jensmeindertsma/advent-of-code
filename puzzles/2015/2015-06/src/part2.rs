use crate::{
    grid::Grid,
    instruction::{Instruction, Task},
};

pub fn part_2(input: &str) -> usize {
    let mut grid = Grid::new(1000, 0usize);

    grid.process(
        input
            .trim()
            .lines()
            .map(|line| Instruction::parse(line).unwrap()),
        |task, light| match task {
            Task::Toggle => *light += 2,
            Task::Enable => *light += 1,
            Task::Disable => *light = light.saturating_sub(1),
        },
    );

    grid.columns().flat_map(|column| column.iter()).sum()
}
