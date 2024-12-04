mod part1;

pub use part1::part_1;

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use super::part_1;

        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);

        assert_eq!(part_1(INPUT), 2572);
    }
}
