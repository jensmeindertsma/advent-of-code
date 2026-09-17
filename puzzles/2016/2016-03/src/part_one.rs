use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.trim().parse::<u16>().unwrap())
                .collect()
        })
        .filter(|sides: &Vec<u16>| {
            [sides[0], sides[1], sides[2]]
                .iter()
                .permutations(3)
                .all(|sides| sides[0] + sides[1] > *sides[2])
        })
        .count()
}
