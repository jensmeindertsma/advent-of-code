use crate::present::Present;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(Present::try_from)
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
