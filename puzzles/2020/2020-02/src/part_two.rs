use crate::parsing::parse;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| parse(line.trim()).unwrap().1)
        .filter(|(position_a, position_b, letter, password)| {
            (password.chars().nth(position_a - 1).unwrap() == *letter)
                ^ (password.chars().nth(position_b - 1).unwrap() == *letter)
        })
        .count()
}
