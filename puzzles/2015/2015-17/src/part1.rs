use itertools::Itertools;

pub fn part_1(input: &str, liters: usize) -> usize {
    let containers: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    (1..=containers.len())
        .flat_map(|size| {
            containers
                .iter()
                .combinations(size)
                .map(|combination| combination.into_iter().copied().sum())
        })
        .filter(|sum: &usize| *sum == liters)
        .count()
}
