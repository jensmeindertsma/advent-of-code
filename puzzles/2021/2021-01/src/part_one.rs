use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}
