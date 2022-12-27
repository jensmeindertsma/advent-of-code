use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let count = count_tail_positions(&input);

    println!("The tail visited {count} positions at least once.");
}

fn count_tail_positions(input: &str) -> usize {
    let motions: Vec<Motion> = input
        .lines()
        .map(|line| {
            let mut slices = line.split_whitespace();

            Motion {
                direction: match slices.next().unwrap() {
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    "U" => Direction::Up,
                    other => panic!("Unexpected direction `{other}`"),
                },
                steps: slices.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut rope = Rope::new();

    motions.iter().for_each(|motion| {
        for _ in 0..motion.steps {
            rope.move_head(&motion.direction)
        }
    });

    rope.visited.len()
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: u8,
}

#[derive(Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

struct Rope {
    visited: HashSet<Position>,
    head: Position,
    tail: Position,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Rope {
    fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert(Position::default());

        Self {
            visited,
            head: Position::default(),
            tail: Position::default(),
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
            Direction::Up => self.head.y -= 1,
        }

        if ![
            (self.head.x - 1, self.head.y - 1).into(),
            (self.head.x, self.head.y - 1).into(),
            (self.head.x + 1, self.head.y - 1).into(),
            (self.head.x - 1, self.head.y).into(),
            self.head.clone(),
            (self.head.x + 1, self.head.y).into(),
            (self.head.x - 1, self.head.y + 1).into(),
            (self.head.x, self.head.y + 1).into(),
            (self.head.x + 1, self.head.y + 1).into(),
        ]
        .iter()
        .any(|position| *position == self.tail)
        {
            let mut tail = self.head.clone();

            // New position for tail should be behind head, based on direction
            // the head just moved.
            match direction {
                Direction::Down => tail.y -= 1,
                Direction::Left => tail.x += 1,
                Direction::Right => tail.x -= 1,
                Direction::Up => tail.y += 1,
            }

            self.tail = tail.clone();
            self.visited.insert(tail);
        }
    }
}

#[cfg(test)]
mod tests {
    use day_09::TEST_INPUT;

    use super::count_tail_positions;

    #[test]
    fn test_input() {
        assert_eq!(count_tail_positions(TEST_INPUT), 13)
    }
}
