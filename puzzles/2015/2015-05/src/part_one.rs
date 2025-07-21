use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let vowel_count = line
                .chars()
                .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count();

            let has_repeating_letter = line.chars().tuple_windows().any(|(a, b)| a == b);

            let has_illegal_string = ["ab", "cd", "pq", "xy"].iter().any(|s| line.contains(s));

            vowel_count >= 3 && has_repeating_letter && !has_illegal_string
        })
        .count()
}
