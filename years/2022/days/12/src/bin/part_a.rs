#![feature(iter_next_chunk)]

use std::{fmt, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let pathfinder = Pathfinder::from(input.as_str());

    let steps = pathfinder.run();

    println!("Minimum amount of steps required: {steps}");
}

struct Pathfinder {
    map: Vec<Vec<Square>>,
    position: (usize, usize),
    elevation: usize,
    steps: usize,
}

impl From<&str> for Pathfinder {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<Square>> = value
            .lines()
            .map(|line| line.chars().map(Square::from).collect())
            .collect();

        let position = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|square| *square == Square::Position)
                    .map(|x| (x, y))
            })
            .unwrap();

        Self {
            map,
            position,
            elevation: 0,
            steps: 0,
        }
    }
}

impl fmt::Debug for Pathfinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n--- MAP ---")
            .and_then(|()| {
                writeln!(
                    f,
                    "{}",
                    self.map
                        .iter()
                        .map(|row| format!(
                            "{}\n",
                            row.iter()
                                .map(|square| format!("{:<4}", format!("{square:?},")))
                                .collect::<String>()
                        ))
                        .collect::<String>()
                        .trim_end()
                )
            })
            .and_then(|()| writeln!(f, "--- INTERNAL ---"))
            .and_then(|()| writeln!(f, "Position: {:?}", self.position))
            .and_then(|()| writeln!(f, "Elevation: {:?}", self.elevation))
            .and_then(|()| writeln!(f, "Steps: {:?}", self.steps))
    }
}

impl Pathfinder {
    fn run(self) -> usize {
        todo!("{:?}", self)
    }
}

#[derive(PartialEq)]
enum Square {
    Regular(usize),
    Position,
    Destination,
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Destination => "E".to_owned(),
                Self::Position => "S".to_owned(),
                Self::Regular(height) => height.to_string(),
            }
        )
    }
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Position,
            'E' => Self::Destination,
            other => Self::Regular((other as usize) - 97),
        }
    }
}

#[cfg(test)]
mod tests {
    use day_12::TEST_INPUT;

    use super::Pathfinder;

    #[test]
    fn test_input() {
        let pathfinder = Pathfinder::from(TEST_INPUT);

        assert_eq!(pathfinder.run(), 31)
    }
}
