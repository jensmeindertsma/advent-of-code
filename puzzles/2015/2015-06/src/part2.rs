use crate::common::{Instruction, InstructionKind};

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<Instruction>().unwrap())
        .fold(vec![[0usize; 1000]; 1000], |mut grid, instruction| {
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
                        InstructionKind::Toggle => *light += 2,
                        InstructionKind::Enable => *light += 1,
                        InstructionKind::Disable => *light = light.saturating_sub(1),
                    }
                }
            }

            grid
        })
        .iter()
        .flat_map(|column| column.iter())
        .sum()
}
