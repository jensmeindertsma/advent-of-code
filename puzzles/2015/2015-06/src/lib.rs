mod grid;
mod instruction;
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

        assert_eq!(part_1("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(
            part_1(
                "
                turn on 0,0 through 999,999
                turn off 499,499 through 500,500
                "
            ),
            1_000_000 - 4
        );

        assert_eq!(part_1(INPUT), 377891);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2("turn on 0,0 through 0,0"), 1);
        assert_eq!(part_2("toggle 0,0 through 999,999"), 2_000_000);

        assert_eq!(part_2(INPUT), 14110788);
    }
}
