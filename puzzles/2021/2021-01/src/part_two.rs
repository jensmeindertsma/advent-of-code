use itertools::Itertools;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(a, b, c, d)| (b + c + d) > (a + b + c))
        .count()
}
