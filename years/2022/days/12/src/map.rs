use crate::position::Position;
use std::fmt;

pub struct Map {
    grid: Vec<Vec<Position>>,
}

impl Map {
    pub fn start(&self) -> (usize, usize) {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|position| position == &Position::Start)
                    .map(|x| (x, y))
            })
            .unwrap()
    }

    pub fn destination(&self) -> (usize, usize) {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|position| position == &Position::Destination)
                    .map(|x| (x, y))
            })
            .unwrap()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let grid: Vec<Vec<Position>> = value
            .lines()
            .map(|line| line.chars().map(Position::from).collect())
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
