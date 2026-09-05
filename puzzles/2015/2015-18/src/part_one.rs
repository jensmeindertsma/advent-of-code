use std::mem;

pub fn part_one(input: &str, steps: usize) -> usize {
    let lines: Vec<&str> = input.trim().lines().collect();

    let height = lines.len();
    let width = lines[0].len();

    // We use a padded grid with disabled lights around the border. This saves
    // us from having to do bounds checking for each access.
    let mut grid = vec![vec![Light::Off; height + 2]; width + 2];
    let mut next = grid.clone();

    for (row, line) in lines.iter().enumerate() {
        for (column, character) in line.chars().enumerate() {
            grid[row + 1][column + 1] = match character {
                '#' => Light::On,
                '.' => Light::Off,
                c => panic!("unexpected character '{c}'"),
            };
        }
    }

    for _ in 0..steps {
        for row in 1..=height {
            for column in 1..=width {
                let neighbors = [
                    grid[row - 1][column - 1],
                    grid[row - 1][column],
                    grid[row - 1][column + 1],
                    grid[row][column - 1],
                    grid[row][column + 1],
                    grid[row + 1][column - 1],
                    grid[row + 1][column],
                    grid[row + 1][column + 1],
                ];

                let lit = neighbors
                    .iter()
                    .filter(|light| **light == Light::On)
                    .count();

                next[row][column] = match grid[row][column] {
                    Light::On if lit == 2 || lit == 3 => Light::On,
                    Light::Off if lit == 3 => Light::On,
                    _ => Light::Off,
                };
            }
        }

        mem::swap(&mut grid, &mut next);
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|light| **light == Light::On)
        .count()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Light {
    On,
    Off,
}
