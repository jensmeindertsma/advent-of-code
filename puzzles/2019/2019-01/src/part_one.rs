pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mass: f32 = line.parse().unwrap();
            let factor = 3.0;

            let result = mass / factor;

            result.floor() as usize - 2
        })
        .sum()
}
