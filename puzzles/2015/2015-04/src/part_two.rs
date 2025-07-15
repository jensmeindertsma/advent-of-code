pub fn part_two(input: &str) -> u32 {
    input.trim().chars().map(|c| c.to_digit(10).unwrap()).sum()
}
