use crate::parsing;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| parsing::present(line).expect("parsing should succeed"))
        .map(|present| {
            let sides = present.sides().map(|s| s.area());
            sides.iter().sum::<usize>()
                + sides
                    .iter()
                    .min()
                    .expect("one of the sides should be the smallest")
        })
        .sum()
}
