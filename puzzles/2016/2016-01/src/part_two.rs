use crate::navigation::{Direction, Heading};
use std::collections::HashSet;

pub fn part_two(input: &str) -> usize {
    let instructions = input.trim().split(", ").map(|instruction| {
        let (direction, distance) = instruction.split_at(1);

        (
            match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            },
            distance.parse::<u8>().unwrap(),
        )
    });

    let mut heading = Heading::North;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for (direction, distance) in instructions {
        heading = heading.turn(direction);

        for _ in 1..=distance {
            match heading {
                Heading::North => y += 1,
                Heading::East => x += 1,
                Heading::South => y -= 1,
                Heading::West => x -= 1,
            }

            let new = visited.insert((x, y));

            if !new {
                return (x.abs() + y.abs()) as usize;
            }
        }
    }

    (x.abs() + y.abs()) as usize
}
