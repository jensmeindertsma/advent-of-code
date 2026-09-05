use itertools::Itertools;

pub fn part_two(input: &str, liters: usize) -> usize {
    let containers: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let combinations: Vec<(usize, usize)> = (1..=containers.len())
        .flat_map(|k| containers.iter().combinations(k))
        .map(|combination| (combination.len(), combination.into_iter().sum::<usize>()))
        .filter(|(_, sum)| *sum == liters)
        .collect();

    let minimum_size = combinations.iter().map(|(length, _)| length).min().unwrap();

    combinations
        .iter()
        .filter(|(length, _)| length == minimum_size)
        .count()
}
