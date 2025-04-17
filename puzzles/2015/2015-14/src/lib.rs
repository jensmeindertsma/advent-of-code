mod part1;
mod part2;
mod reindeer;

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
                Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
                ",
                1000
            ),
            1120
        );

        assert_eq!(part_1(INPUT, 2503), 2696);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(
            part_2(
                "
                Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
                ",
                1000
            ),
            689
        );

        assert_eq!(part_2(INPUT, 2503), 1084);
    }
}
