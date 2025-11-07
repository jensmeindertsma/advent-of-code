use crate::parsing;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parsing::present)
        .map(|present| {
            present.sides().map(|s| s.perimeter()).iter().min().unwrap() + present.volume()
        })
        .sum()
}
