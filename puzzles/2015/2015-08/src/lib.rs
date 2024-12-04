mod part1;
mod part2;

pub use part1::part_1;
pub use part2::part_2;

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(
            part_1(
                "
                \"\"
                \"abc\"
                \"aaa\\\"aaa\"
                \"\\x27\"
                "
            ),
            12
        );

        assert_eq!(part_1(INPUT), 1350);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2("\"\""), 6 - 2);
        assert_eq!(part_2("\"abc\""), 9 - 5);
        assert_eq!(part_2("\"aaa\\\"aaa\""), 16 - 10);
        assert_eq!(part_2("\"\\x27\""), 11 - 6);

        assert_eq!(part_2(INPUT), 2085);
    }
}
