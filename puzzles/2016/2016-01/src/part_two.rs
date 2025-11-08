use std::collections::HashSet;

use crate::{
    parsing::parse,
    shared::{Heading, Position, Turn},
};

pub fn part_two(input: &str) -> usize {
    let mut position = Position { x: 0, y: 0 };
    let mut heading = Heading::North;

    let mut visited = HashSet::new();

    for (turn, distance) in parse(input) {
        heading = match turn {
            Turn::Left => heading.left(),
            Turn::Right => heading.right(),
        };

        // Take baby steps travelling the distance
        for _ in 1..=distance {
            position.travel(heading, 1);

            let new = visited.insert(position);

            // So we can check for second visit during every step

            if !new {
                return position.x.unsigned_abs() + position.y.unsigned_abs();
            }
        }
    }

    unreachable!()
}
