use std::collections::HashSet;

pub fn part_two(input: &str) -> isize {
    let changes = input
        .trim()
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|line| line.trim().parse::<isize>().unwrap())
        .cycle();

    let mut current_frequency = 0;
    let mut visited_frequencies = HashSet::new();

    visited_frequencies.insert(current_frequency);

    for change in changes {
        current_frequency += change;

        let is_new = visited_frequencies.insert(current_frequency);

        if !is_new {
            return current_frequency;
        }
    }

    unreachable!()
}
