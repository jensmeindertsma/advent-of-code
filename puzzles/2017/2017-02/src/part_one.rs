pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let min: usize = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .min()
                .unwrap();

            let max: usize = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .max()
                .unwrap();

            max - min
        })
        .sum()
}
