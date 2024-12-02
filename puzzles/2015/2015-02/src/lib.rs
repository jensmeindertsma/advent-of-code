mod common;
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

        assert_eq!(part_1("2x3x4"), 58);
        assert_eq!(part_1("1x1x10"), 43);

        assert_eq!(part_1(INPUT), 1588178);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2("2x3x4"), 34);
        assert_eq!(part_2("1x1x10"), 14);

        assert_eq!(part_2(INPUT), 3783758);
    }
}
