pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|string| {
            string
                .chars()
                .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count()
                >= 3
        })
        .filter(|string| string.as_bytes().windows(2).any(|w| w[0] == w[1]))
        .filter(|string| {
            string
                .as_bytes()
                .windows(2)
                .all(|w| !matches!(w, b"ab" | b"cd" | b"pq" | b"xy"))
        })
        .count()
}
