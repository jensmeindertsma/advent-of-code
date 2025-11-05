pub fn part_two(input: &str) -> usize {
    input.trim().chars().filter(|c| *c == 'a').count()
}
