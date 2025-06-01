use itertools::Itertools;

pub fn part_2(input: &str, liters: usize) -> usize {
    let containers: Vec<usize> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    (1..=containers.len())
        .flat_map(|size| containers.iter().combinations(size))
        .filter(|combination| combination.iter().copied().sum::<usize>() == liters)
        .map(|combination| combination.len())
        .chunk_by(|size| *size)
        // Now we have groups with multiple "elements" all of which
        // are the same: the size of the combination.
        .into_iter()
        // To calculate the amount of different possible combination
        // we just need to count the elements in the group.
        .map(|(_, group)| group.count())
        .min()
        .unwrap()
}
