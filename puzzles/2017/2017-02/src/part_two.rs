use itertools::Itertools;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .permutations(2)
                .find_map(|pair: Vec<usize>| {
                    if pair[0].is_multiple_of(pair[1]) {
                        Some(pair[0] / pair[1])
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum()
}
