pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut encoded_string = String::new();
            encoded_string.push('"');

            for character in line.chars() {
                match character {
                    '"' => encoded_string.push_str("\\\""),
                    '\\' => encoded_string.push_str("\\\\"),
                    _ => encoded_string.push(character),
                }
            }

            encoded_string.push('"');

            encoded_string.len() - line.len()
        })
        .sum()
}
