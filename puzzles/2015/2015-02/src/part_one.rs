use crate::parsing;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, present) = parsing::present(line).unwrap();
            present
        })
        .map(|present| {
            let sides = present.sides().map(|s| s.area());
            sides.iter().sum::<usize>() + sides.iter().min().unwrap()
        })
        .sum()
}
