use std::str::FromStr;

use crate::present::Present;

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(Present::from_str)
        .map(|result| result.expect("dimensions should always be valid"))
        .map(|present| {
            present.volume()
                + present
                    .sides()
                    .iter()
                    .map(|(x, y)| 2 * x + 2 * y)
                    .min()
                    .expect("a present should always have sides")
        })
        .sum()
}
