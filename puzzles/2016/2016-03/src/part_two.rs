use itertools::Itertools;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let sides: Vec<u16> = line
                .split_whitespace()
                .map(|value| value.trim().parse::<u16>().unwrap())
                .collect();

            sides
        })
        .tuples::<(Vec<u16>, Vec<u16>, Vec<u16>)>()
        .flat_map(|(a, b, c)| (0..3).map(move |i| (a[i], b[i], c[i])))
        .filter(|(a, b, c)| {
            [a, b, c]
                .iter()
                .permutations(3)
                .all(|sides| *sides[0] + *sides[1] > **sides[2])
        })
        .count()
}
