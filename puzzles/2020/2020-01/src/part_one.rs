use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .combinations(2)
        .find(|combination| combination[0] + combination[1] == 2020)
        .map(|combination| combination[0] * combination[1])
        .unwrap()
}
