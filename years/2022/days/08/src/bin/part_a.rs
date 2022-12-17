use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let count = count_visible_trees(&input);

    println!("Amount of visible trees: {count}");
}

fn count_visible_trees(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, height)| {
                    // Check if tree is visible
                    let above: Vec<u8> = grid[..y].iter().map(|row| row[*x]).rev().collect();
                    let below: Vec<u8> = grid[(y + 1)..].iter().map(|row| row[*x]).collect();
                    let left = &row[..*x].iter().rev().copied().collect();
                    let right = &row[(*x + 1)..];

                    [&above, &below, left, right].iter().any(|direction| {
                        if direction.is_empty() {
                            true
                        } else {
                            direction.iter().all(|obstacle| obstacle < height)
                        }
                    })
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day_08::TEST_INPUT;

    use super::count_visible_trees;

    #[test]
    fn test_input() {
        assert_eq!(count_visible_trees(TEST_INPUT), 21)
    }
}
