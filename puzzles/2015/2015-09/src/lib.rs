mod part1;
mod part2;
mod router;

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
                London to Dublin = 464
                London to Belfast = 518
                Dublin to Belfast = 141
                "
            ),
            605
        );

        assert_eq!(part_1(INPUT), 251);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(
            part_2(
                "
                London to Dublin = 464
                London to Belfast = 518
                Dublin to Belfast = 141
                "
            ),
            982
        );

        assert_eq!(part_2(INPUT), 898);
    }
}
