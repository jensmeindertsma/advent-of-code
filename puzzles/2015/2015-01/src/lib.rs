mod part_one;
mod part_two;

pub use part_one::part_1;
pub use part_two::part_2;

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use super::part_1;

        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);

        assert_eq!(part_1(INPUT), 138);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
        assert_eq!(part_2(INPUT), 1771);
    }
}
