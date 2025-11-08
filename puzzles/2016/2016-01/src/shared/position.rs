use crate::shared::Heading;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn travel(&mut self, heading: Heading, distance: usize) {
        // We assume distance <= isize::MAX
        let distance = distance as isize;

        match heading {
            Heading::East => self.x += distance,
            Heading::North => self.y += distance,
            Heading::South => self.y -= distance,
            Heading::West => self.x -= distance,
        };
    }
}
