use std::str::FromStr;

use crate::present::Present;

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(Present::from_str)
        .map(|result| result.expect("dimensions should always be valid"))
        .map(|present| {
            present.surface_area()
                + present
                    .sides()
                    .iter()
                    .map(|(x, y)| x * y)
                    .min()
                    .expect("a present should always have sides")
        })
        .sum()
}
