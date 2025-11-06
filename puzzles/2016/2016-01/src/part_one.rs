use crate::{direction::Direction, instruction::Instruction, parsing};

pub fn part_one(input: &str) -> usize {
    let mut direction = Direction::North;
    let mut x: isize = 0;
    let mut y: isize = 0;

    let (_, instructions) = parsing::document(input).unwrap();

    for instruction in instructions {
        let distance = match instruction {
            Instruction::Left(distance) => {
                direction = direction.left();
                distance
            }
            Instruction::Right(distance) => {
                direction = direction.right();
                distance
            }
        };

        match direction {
            Direction::East => x += distance as isize,
            Direction::North => y += distance as isize,
            Direction::South => y -= distance as isize,
            Direction::West => x -= distance as isize,
        }
    }

    (x.abs() + y.abs()) as usize
}
