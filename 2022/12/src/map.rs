use crate::place::Place;
use std::fmt;

pub struct Map {
    grid: Vec<Vec<Place>>,
}

impl Map {
    pub fn start(&self) -> Position {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|position| position == &Place::Start)
                    .map(|x| Position { x, y })
            })
            .unwrap()
    }

    pub fn destination(&self) -> Position {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|position| position == &Place::Destination)
                    .map(|x| Position { x, y })
            })
            .unwrap()
    }

    pub fn surrounding(&self, position: Position) -> &[Place] {
        let place = self.grid[position.y][position.x];

        let 
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let grid: Vec<Vec<Place>> = value
            .lines()
            .map(|line| line.chars().map(Place::from).collect())
            .collect();

        Self { grid }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "--- MAP --- --- {} by {} ---",
            self.grid.len(),
            self.grid[0].len()
        )
        .and_then(|_| {
            write!(
                f,
                "{}",
                self.grid
                    .iter()
                    .map(|row| format!(
                        "{}\n",
                        row.iter()
                            .map(|position| format!("{:<4}", format!("{position:?}")))
                            .collect::<String>()
                    ))
                    .collect::<String>()
            )
        })
    }
}

#[derive(PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}
