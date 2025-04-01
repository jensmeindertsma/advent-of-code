mod part1;

pub use part1::part_1;

#[cfg(test)]
mod tests {
    use std::time::Duration;

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
                Duration::from_secs(1000)
            ),
            120
        );

        assert_eq!(part_1(INPUT, Duration::from_secs(2503)), 2696);
    }
}
