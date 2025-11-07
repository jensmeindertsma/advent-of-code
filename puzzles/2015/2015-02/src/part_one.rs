use crate::parsing;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parsing::present)
        .map(|present| {
            let sides = present.sides().map(|s| s.area());
            sides.iter().sum::<usize>() + sides.iter().min().unwrap()
        })
        .sum()
}
