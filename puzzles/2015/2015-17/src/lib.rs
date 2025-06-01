mod part1;
mod part2;

pub use part1::part_1;
pub use part2::part_2;

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use super::part_1;

        assert_eq!(
            part_1(
                "
                20
                15
                10
                5
                5
                ",
                25
            ),
            4
        );

        assert_eq!(part_1(INPUT, 150), 1304);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2(INPUT, 150), 18);
    }
}
