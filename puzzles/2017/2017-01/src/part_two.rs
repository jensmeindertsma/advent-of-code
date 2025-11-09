pub fn part_two(input: &str) -> usize {
    let input = input.trim();
    let len = input.len();
    let step = len / 2;

    input
        .chars()
        // zip together yields `(character, character len/2 forward)`
        .zip(input.chars().cycle().skip(step))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10).unwrap() as usize)
        .sum()
}
