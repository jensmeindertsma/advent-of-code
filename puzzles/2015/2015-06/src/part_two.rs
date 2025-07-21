use crate::parsing::{self, Instruction, Operation};

pub fn part_two(input: &str) -> usize {
    #[derive(Clone, Copy)]
    struct Light {
        brightness: usize,
    }

    let grid: [Vec<Light>; 1000] = std::array::from_fn(|_| vec![Light { brightness: 0 }; 1000]);

    input
        .trim()
        .lines()
        .fold(grid, |mut grid, line| {
            let Instruction { from, to, kind } = parsing::parse(line.trim());

            for row in &mut grid[from.y..=to.y] {
                for cell in &mut row[from.x..=to.x] {
                    match kind {
                        Operation::Toggle => cell.brightness += 2,
                        Operation::TurnOff => cell.brightness = cell.brightness.saturating_sub(1),
                        Operation::TurnOn => cell.brightness += 1,
                    }
                }
            }

            grid
        })
        .map(|row| row.iter().map(|l| l.brightness).sum())
        .iter()
        .sum()
}
