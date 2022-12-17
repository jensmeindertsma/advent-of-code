#![feature(array_windows)]

use lending_iterator::prelude::*;
use std::{collections::HashSet, fmt, fs};

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

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("({},{})", self.x, self.y))
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

struct Rope {
    visited: HashSet<Position>,
    knots: [Position; 10],
}

impl Rope {
    fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert(Position::default());

        Self {
            visited,
            knots: [Position::default(); 10],
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Down => self.knots[0].y += 1,
            Direction::Left => self.knots[0].x -= 1,
            Direction::Right => self.knots[0].x += 1,
            Direction::Up => self.knots[0].y -= 1,
        }

        let mut windows = self.knots.windows_mut::<2>();
        while let Some([start, ref mut end]) = windows.next() {
            if ![
                (start.x - 1, start.y - 1).into(),
                (start.x, start.y - 1).into(),
                (start.x + 1, start.y - 1).into(),
                (start.x - 1, start.y).into(),
                *start,
                (start.x + 1, start.y).into(),
                (start.x - 1, start.y + 1).into(),
                (start.x, start.y + 1).into(),
                (start.x + 1, start.y + 1).into(),
            ]
            .iter()
            .any(|position| position == end)
            {
                if start.x == end.x {
                    if start.y > end.y {
                        end.y += 1;
                    } else {
                        end.y -= 1;
                    }
                } else if start.y == end.y {
                    if start.x > end.x {
                        end.x += 1;
                    } else {
                        end.x -= 1;
                    }
                } else {
                    let end_positions = [
                        (end.x - 1, end.y - 1).into(),
                        (end.x, end.y - 1).into(),
                        (end.x + 1, end.y - 1).into(),
                        (end.x - 1, end.y).into(),
                        *end,
                        (end.x + 1, end.y).into(),
                        (end.x - 1, end.y + 1).into(),
                        (end.x, end.y + 1).into(),
                        (end.x + 1, end.y + 1).into(),
                    ];

                    let overlapping_positions: Vec<&Position> = end_positions
                        .iter()
                        .filter(|end_position| {
                            [
                                (start.x - 1, start.y - 1).into(),
                                (start.x, start.y - 1).into(),
                                (start.x + 1, start.y - 1).into(),
                                (start.x - 1, start.y).into(),
                                *start,
                                (start.x + 1, start.y).into(),
                                (start.x - 1, start.y + 1).into(),
                                (start.x, start.y + 1).into(),
                                (start.x + 1, start.y + 1).into(),
                            ]
                            .contains(end_position)
                        })
                        .collect();

                    *end = match overlapping_positions.len() {
                        // 1 means a corner overlaps, we take that.
                        1 => *overlapping_positions[0],
                        // 2 means a corner and a side overlap, we take the side.
                        2 => {
                            let start_sides: [Position; 4] = [
                                (start.x - 1, start.y).into(),
                                (start.x + 1, start.y).into(),
                                (start.x, start.y - 1).into(),
                                (start.x, start.y + 1).into(),
                            ];
                            let new_end_position = overlapping_positions
                                .iter()
                                .find(|tuple| start_sides.contains(tuple))
                                .unwrap();

                            **new_end_position
                        }
                        n => {
                            println!("{start:?}, {end:?}");
                            panic!("unknown tail length: {n}");
                        }
                    };
                }
            }
        }

        self.visited.insert(*self.knots.last().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use day_09::TEST_INPUT;

    use super::count_tail_positions;

    #[test]
    fn test_input() {
        assert_eq!(count_tail_positions(TEST_INPUT), 1)
    }

    #[test]
    fn test_input_2() {
        const TEST_INPUT: &str = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";

        assert_eq!(count_tail_positions(TEST_INPUT), 36);
    }
}
