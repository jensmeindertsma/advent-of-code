#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    pub fn turn(self, direction: Direction) -> Self {
        match direction {
            Direction::Left => match self {
                Heading::North => Heading::West,
                Heading::West => Heading::South,
                Heading::South => Heading::East,
                Heading::East => Heading::North,
            },
            Direction::Right => match self {
                Heading::North => Heading::East,
                Heading::East => Heading::South,
                Heading::South => Heading::West,
                Heading::West => Heading::North,
            },
        }
    }
}
