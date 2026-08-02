use itertools::Itertools;

pub fn part_one(input: &str, liters: usize) -> usize {
    let containers: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (1..=containers.len())
        .flat_map(|k| containers.iter().combinations(k))
        .map(|combination| combination.into_iter().sum())
        .filter(|sum: &usize| *sum == liters)
        .count()
}
