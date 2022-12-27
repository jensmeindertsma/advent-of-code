use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let score = find_maximum_scenic_score(&input);

    println!("Maximum scenic score: {score}");
}

fn find_maximum_scenic_score(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut scores: Vec<usize> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, height)| {
                    let above: Vec<u8> = grid[..y].iter().map(|row| row[x]).rev().collect();
                    let below: Vec<u8> = grid[(y + 1)..].iter().map(|row| row[x]).collect();
                    let left = &row[..x].iter().rev().copied().collect();
                    let right = &row[(x + 1)..];

                    [&above, &below, left, right]
                        .iter()
                        .map(|direction| {
                            direction
                                .iter()
                                .position(|obstacle| obstacle >= height)
                                .map_or(direction.len(), |distance| distance + 1)
                        })
                        .reduce(|total, score| total * score)
                        .unwrap()
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    scores.sort();

    *scores.last().unwrap()
}

#[cfg(test)]
mod tests {
    use day_08::TEST_INPUT;

    use super::find_maximum_scenic_score;

    #[test]
    fn test_input() {
        assert_eq!(find_maximum_scenic_score(TEST_INPUT), 8)
    }
}
