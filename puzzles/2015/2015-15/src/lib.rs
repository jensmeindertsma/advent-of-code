mod ingredient;
mod mixer;
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

        assert_eq!(
            part_1(
                "
                Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
                "
            ),
            62842880
        );

        assert_eq!(part_1(INPUT), 21367368);
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(
            part_2(
                "
                Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
                "
            ),
            57600000
        );

        assert_eq!(part_2(INPUT), 1766400);
    }
}
