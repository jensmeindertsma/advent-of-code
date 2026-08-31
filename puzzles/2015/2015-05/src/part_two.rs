pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|string| {
            string.char_indices().any(|(index, _)| {
                // If this is the last character we should stop
                // because this cannot become a new pair.
                if index + 1 >= string.len() {
                    return false;
                }

                let pair = &string[index..index + 2];

                let before = &string[..index];
                let after = &string[index + 2..];

                before.contains(pair) || after.contains(pair)
            })
        })
        .filter(|string| string.as_bytes().windows(3).any(|w| w[0] == w[2]))
        .count()
}
