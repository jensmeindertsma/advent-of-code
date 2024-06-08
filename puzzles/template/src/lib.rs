pub fn part_1(input: &str) -> usize {
    input.trim().len()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("abc"), 3);
        assert_eq!(part_1("9876543210"), 10);

        assert_eq!(part_1(INPUT), 6);
    }
}
