use itertools::Itertools;

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let contains_pair = line.char_indices().any(|(index, _)| {
                if index + 1 >= line.len() {
                    return false;
                }

                let pair = &line[index..=(index + 1)];

                let before = &line[..index];
                let after = &line[index + 2..];

                before.contains(pair) || after.contains(pair)
            });

            let contains_repeating_spaced_letter =
                line.chars().tuple_windows().any(|(a, _, c)| a == c);

            contains_pair && contains_repeating_spaced_letter
        })
        .count()
}
