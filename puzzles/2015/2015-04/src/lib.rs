mod common;
mod part1;
mod part2;

pub use part1::part_1;
pub use part2::part_2;

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    mod part_1 {
        use crate::part_1;
        use super::INPUT;

        #[test]
        fn examples() {
            assert_eq!(part_1("abcdef"), 609043);
            assert_eq!(part_1("pqrstuv"), 1048970);
        }

        #[test]
        fn solution() {
            assert_eq!(part_1(INPUT), 254575);
        }
    }

    #[test]
    fn part_2() {
        use super::part_2;

        assert_eq!(part_2(INPUT), 1038736);
    }
}
