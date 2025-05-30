mod part1;
mod part2;
mod sequencer;

pub use part1::part_1;
pub use part2::part_2;

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use super::part_1;

        assert_eq!(part_1(INPUT), 329356);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2(INPUT), 4666278);
    }
}
