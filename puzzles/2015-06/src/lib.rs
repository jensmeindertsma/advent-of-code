mod grid;
mod instruction;
mod one;
mod parsing;
mod two;

use grid::Grid;
use instruction::Instruction;

pub fn part_1(input: &str) -> usize {
    use one::Light;

    input
        .trim()
        .lines()
        .map(|line| {
            Instruction::try_from(line).expect("input should include only valid instructions")
        })
        .fold(
            Grid::with_size(1000, Light::default()),
            |mut grid, instruction| {
                grid.instruct(instruction);
                grid
            },
        )
        .lit()
}

pub fn part_2(input: &str) -> usize {
    use two::Light;

    input
        .trim()
        .lines()
        .map(|line| {
            Instruction::try_from(line).expect("input should include only valid instructions")
        })
        .fold(
            Grid::with_size(1000, Light::default()),
            |mut grid, instruction| {
                grid.instruct(instruction);
                grid
            },
        )
        .brightness()
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::{Coordinate, InstructionKind},
        Grid, Instruction,
    };

    const INPUT: &str = include_str!("../input.txt");

    mod part_1 {
        use super::{Coordinate, Grid, Instruction, InstructionKind, INPUT};
        use crate::{one::Light, part_1};

        #[test]
        fn turn_on_all() -> Result<(), &'static str> {
            let mut grid = Grid::with_size(1000, Light::default());

            assert_eq!(grid.lit(), 0);

            let instruction = Instruction::try_from("turn on 0,0 through 999,999")?;

            grid.instruct(instruction);

            assert_eq!(grid.lit(), 1_000_000);

            Ok(())
        }

        #[test]
        fn toggle_top_row() -> Result<(), &'static str> {
            let mut grid = Grid::with_size(1000, Light::default());
            let instruction = Instruction::try_from("toggle 0,0 through 999,0")?;

            grid.instruct(instruction);

            let mut iterator = grid.iter();

            for _ in 0..1000 {
                assert_eq!(iterator.next(), Some(Light::On).as_ref())
            }

            assert!(iterator.all(|l| *l == Light::Off));

            Ok(())
        }

        #[test]
        fn turn_off_middle_four() -> Result<(), &'static str> {
            let mut grid = Grid::with_size(1000, Light::default());

            grid.instruct(Instruction {
                kind: InstructionKind::TurnOn,
                from: Coordinate::new(0, 0),
                to: Coordinate::new(999, 999),
            });

            let instruction = Instruction::try_from("turn off 499,499 through 500,500")?;

            grid.instruct(instruction);

            assert_eq!(grid.lit(), 1_000_000 - 4);

            Ok(())
        }

        #[test]
        fn with_puzzle_input() {
            assert_eq!(part_1(INPUT), 377891);
        }
    }

    mod part_2 {
        use super::{Grid, Instruction, INPUT};
        use crate::{part_2, two::Light};

        #[test]
        fn turn_on_one() -> Result<(), &'static str> {
            let mut grid = Grid::with_size(1000, Light::default());

            let instruction = Instruction::try_from("turn on 0,0 through 0,0")?;

            grid.instruct(instruction);

            assert_eq!(grid.brightness(), 1);

            Ok(())
        }

        #[test]
        fn toggle_million_lights() -> Result<(), &'static str> {
            let mut grid = Grid::with_size(1000, Light::default());

            let instruction = Instruction::try_from("toggle 0,0 through 999,999")?;

            grid.instruct(instruction);

            assert_eq!(grid.brightness(), 2_000_000);

            Ok(())
        }

        #[test]
        fn with_puzzle_input() {
            assert_eq!(part_2(INPUT), 14110788);
        }
    }
}
