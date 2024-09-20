#![allow(clippy::nonminimal_bool)]

pub fn part_1(input: &str, steps: usize) -> usize {
    let mut grid: Vec<Vec<Light>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Light::On,
                    '.' => Light::Off,
                    c => panic!("Unexpected char `{c}`"),
                })
                .collect::<Vec<Light>>()
        })
        .collect();

    for _ in 0..steps {
        let grid_snapshot: Vec<Vec<Light>> = grid.clone();

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, new_light) in row.iter_mut().enumerate() {
                let light = &grid_snapshot[y][x];
                let neighbors_lit = [
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x - 1, y),
                    // Omit light in the middle
                    (x + 1, y),
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                ]
                .iter()
                .map(|(x, y)| {
                    grid_snapshot
                        .get(*y)
                        .and_then(|row| row.get(*x).copied())
                        .unwrap_or(Light::Off)
                })
                .filter(|l| *l == Light::On)
                .count();

                *new_light = match light {
                    Light::On => {
                        if neighbors_lit == 2 || neighbors_lit == 3 {
                            Light::On
                        } else {
                            Light::Off
                        }
                    }
                    Light::Off => {
                        if neighbors_lit == 3 {
                            Light::On
                        } else {
                            Light::Off
                        }
                    }
                }
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|l| **l == Light::On).count())
        .sum()
}

pub fn part_2(input: &str, steps: usize) -> usize {
    let mut grid: Vec<Vec<Light>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Light::On,
                    '.' => Light::Off,
                    c => panic!("Unexpected char `{c}`"),
                })
                .collect::<Vec<Light>>()
        })
        .collect();

    for _ in 0..steps {
        let grid_snapshot: Vec<Vec<Light>> = grid.clone();

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, new_light) in row.iter_mut().enumerate() {
                let light = &grid_snapshot[y][x];

                if is_corner(x, y, grid_snapshot.as_slice()) {
                    *new_light = Light::On
                } else {
                    let neighbors_lit = [
                        (x - 1, y + 1),
                        (x, y + 1),
                        (x + 1, y + 1),
                        (x - 1, y),
                        // Omit light in the middle
                        (x + 1, y),
                        (x - 1, y - 1),
                        (x, y - 1),
                        (x + 1, y - 1),
                    ]
                    .iter()
                    .map(|(x, y)| {
                        if is_corner(*x, *y, grid_snapshot.as_slice()) {
                            Light::On
                        } else {
                            grid_snapshot
                                .get(*y)
                                .and_then(|row| row.get(*x).copied())
                                .unwrap_or(Light::Off)
                        }
                    })
                    .filter(|l| *l == Light::On)
                    .count();

                    *new_light = match light {
                        Light::On => {
                            if neighbors_lit == 2 || neighbors_lit == 3 {
                                Light::On
                            } else {
                                Light::Off
                            }
                        }
                        Light::Off => {
                            if neighbors_lit == 3 {
                                Light::On
                            } else {
                                Light::Off
                            }
                        }
                    }
                }
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|l| **l == Light::On).count())
        .sum()
}

fn is_corner(x: usize, y: usize, grid: &[Vec<Light>]) -> bool {
    (x == 0 && y == 0)
        || (x == 0 && y == (grid.len() - 1))
        || (x == (grid[0].len() - 1) && y == 0)
        || (x == (grid[0].len() - 1) && y == (grid.len() - 1))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Light {
    On,
    Off,
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    const EXAMPLE_INPUT: &str = "
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    ";

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(EXAMPLE_INPUT, 4), 4);

        assert_eq!(part_1(INPUT, 100), 1061);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(EXAMPLE_INPUT, 5), 17);

        assert_eq!(part_2(INPUT, 100), 1006);
    }
}
