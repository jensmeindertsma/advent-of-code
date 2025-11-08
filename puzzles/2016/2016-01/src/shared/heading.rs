#[derive(Clone, Copy, Debug)]
pub enum Heading {
    East,
    North,
    South,
    West,
}

impl Heading {
    pub fn left(&self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
}

pub enum Turn {
    Left,
    Right,
}
