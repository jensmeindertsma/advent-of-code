use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    let input = input.trim();
    let first = input.chars().next().unwrap();

    input
        .chars()
        .chain(std::iter::once(first))
        .tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10).unwrap())
        .sum()
}
