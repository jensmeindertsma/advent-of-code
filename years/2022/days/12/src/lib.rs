pub const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

pub struct Map {
    grid: Vec<Vec<Position>>,
}

impl Map {
    fn start(&self) -> (usize, usize) {
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

    fn destination(&self) -> (usize, usize) {
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

#[derive(PartialEq)]
enum Position {
    Regular { height: u8 },
    Start,
    Destination,
}

impl From<char> for Position {
    fn from(character: char) -> Self {
        match character {
            'S' => Self::Start,
            'E' => Self::Destination,
            other => Self::Regular {
                height: other as u8,
            },
        }
    }
}

pub struct Pathfinder<'a> {
    map: &'a Map,
}

impl<'a> Pathfinder<'a> {
    pub fn run(self) -> usize {
        let start = self.map.start();
        let destination = self.map.destination();

        println!("Pathfinding from {start:?} to {destination:?}");

        todo!()
    }
}

impl<'a> From<&'a Map> for Pathfinder<'a> {
    fn from(map: &'a Map) -> Self {
        Self { map }
    }
}
