use crate::present::Present;

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(Present::from_description)
        .map(|present| {
            let perimeter = present
                .sides()
                .iter()
                .map(|side| side.perimeter())
                .min()
                .unwrap();

            perimeter + present.volume()
        })
        .sum()
}
