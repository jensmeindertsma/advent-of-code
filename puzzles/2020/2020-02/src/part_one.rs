use crate::parsing::parse;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| parse(line.trim()).unwrap().1)
        .filter(|(at_least, at_most, letter, password)| {
            let count = password.chars().filter(|c| c == letter).count();
            count >= *at_least && count <= *at_most
        })
        .count()
}
