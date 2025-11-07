use crate::parsing;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| parsing::present(line).expect("parsing should succeed"))
        .map(|present| {
            present
                .sides()
                .map(|s| s.perimeter())
                .iter()
                .min()
                .expect("there should be a smallest perimeter")
                + present.volume()
        })
        .sum()
}
