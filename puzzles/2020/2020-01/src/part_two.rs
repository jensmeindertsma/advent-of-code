use itertools::Itertools;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .combinations(3)
        .find(|combination| combination[0] + combination[1] + combination[2] == 2020)
        .map(|combination| combination[0] * combination[1] * combination[2])
        .unwrap()
}
