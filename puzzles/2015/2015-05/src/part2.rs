use itertools::Itertools;

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let string = line.trim();

            let has_double_pair =
                string
                    .chars()
                    .tuple_windows()
                    .enumerate()
                    .any(|(index, (a, b))| {
                        // Get index into string *after* the current pair.
                        let index = index + 2;

                        // Get remaining part of the string *after* current pair
                        if index > string.len() {
                            false
                        } else {
                            let remaining = &string[index..];
                            remaining.contains(&format!("{a}{b}"))
                        }
                    });

            let contains_spaced_repeat = string
                .chars()
                .tuple_windows()
                .any(|(a, b, c)| a != b && a == c);

            has_double_pair && contains_spaced_repeat
        })
        .count()
}
