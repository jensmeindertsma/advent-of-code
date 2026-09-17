use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let sides: Vec<u16> = line
                .split_whitespace()
                .map(|value| value.trim().parse::<u16>().unwrap())
                .collect();

            [sides[0], sides[1], sides[2]]
                .iter()
                .permutations(3)
                .all(|sides| sides[0] + sides[1] > *sides[2])
        })
        .filter(|possible| *possible)
        .count()
}
