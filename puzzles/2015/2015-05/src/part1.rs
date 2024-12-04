use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let string = line.trim();

            let contains_three_vowels = string
                .chars()
                .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count()
                >= 3;

            let contains_consecutive_letter = string.chars().tuple_windows().any(|(a, b)| a == b);

            let contains_illegal = ["ab", "cd", "pq", "xy"].iter().any(|s| string.contains(s));

            contains_three_vowels && contains_consecutive_letter && !contains_illegal
        })
        .count()
}
