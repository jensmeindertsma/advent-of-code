use crate::{
    parsing::parse,
    shared::{Heading, Position, Turn},
};

pub fn part_one(input: &str) -> usize {
    let mut position = Position { x: 0, y: 0 };
    let mut heading = Heading::North;

    for (turn, distance) in parse(input) {
        heading = match turn {
            Turn::Left => heading.left(),
            Turn::Right => heading.right(),
        };

        position.travel(heading, distance);
    }
    // Manhattan distance [https://en.wikipedia.org/wiki/Taxicab_geometry]
    position.x.unsigned_abs() + position.y.unsigned_abs()
}
