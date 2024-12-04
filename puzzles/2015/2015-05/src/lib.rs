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

        assert_eq!(part_1("ugknbfddgicrmopn"), 1);
        assert_eq!(part_1("aaa"), 1);
        assert_eq!(part_1("jchzalrnumimnmhp"), 0);
        assert_eq!(part_1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part_1("dvszwmarrgswjxmb"), 0);

        assert_eq!(part_1(INPUT), 258);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part_2("xxyxx"), 1);
        assert_eq!(part_2("uurcxstgmygtbstg"), 0);
        assert_eq!(part_2("ieodomkazucvgmuy"), 0);

        assert_eq!(part_2(INPUT), 53);
    }
}
