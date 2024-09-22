use core::fmt::{self, Formatter};
use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let starting_location = (0, 0);
    let (x, y) = input
        .trim()
        .split(", ")
        .map(|string| Instruction::try_from(string).unwrap())
        .fold(
            (starting_location, Direction::North),
            |((x, y), mut direction), instruction| {
                let steps = match instruction {
                    Instruction::Left(steps) => {
                        direction.turn_left();
                        steps as isize
                    }
                    Instruction::Right(steps) => {
                        direction.turn_right();
                        steps as isize
                    }
                };

                (
                    match direction {
                        Direction::East => (x + steps, y),
                        Direction::North => (x, y + steps),
                        Direction::South => (x, y - steps),
                        Direction::West => (x - steps, y),
                    },
                    direction,
                )
            },
        )
        .0;

    let distance = x.abs() + y.abs();

    distance.try_into().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut current_location = Location { x: 0, y: 0 };
    let mut direction = Direction::North;
    let mut visited_locations = HashSet::new();
    visited_locations.insert(current_location);

    for instruction in input
        .trim()
        .split(", ")
        .map(|string| Instruction::try_from(string).unwrap())
    {
        let steps = match instruction {
            Instruction::Left(steps) => {
                direction.turn_left();
                steps as isize
            }
            Instruction::Right(steps) => {
                direction.turn_right();
                steps as isize
            }
        };

        for _ in 0..steps {
            match direction {
                Direction::East => current_location.x += 1,
                Direction::North => current_location.y += 1,
                Direction::South => current_location.y -= 1,
                Direction::West => current_location.x -= 1,
            };

            let newly_inserted = visited_locations.insert(current_location);

            if !newly_inserted {
                return (current_location.x.abs() + current_location.y.abs())
                    .try_into()
                    .unwrap();
            }
        }
    }

    println!("{visited_locations:?}");

    unreachable!()
}

enum Instruction {
    Left(u8),
    Right(u8),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Location {
    x: isize,
    y: isize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[derive(Debug)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let n = value[1..].parse().map_err(|_| ())?;

        match &value[..1] {
            "L" => Ok(Self::Left(n)),
            "R" => Ok(Self::Right(n)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("R2, L3"), 5);
        assert_eq!(part_1("R2, R2, R2"), 2);
        assert_eq!(part_1("R5, L5, R5, R3"), 12);

        assert_eq!(part_1(INPUT), 273);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2("R8, R4, R4, R8"), 4);

        assert_eq!(part_2(INPUT), 115);
    }
}
