#![feature(iter_array_chunks)]

use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let sum = calculate_priorities(&input);

    println!("Sum of priorities: {sum}");
}

fn calculate_priorities(input: &str) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, character)| (character, index + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            let common = first
                .chars()
                .find(|character| second.contains(*character) && third.contains(*character))
                .unwrap();

            letter_scores.get(&common).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day_03::TEST_INPUT;

    use super::calculate_priorities;

    #[test]
    fn example_list() {
        assert_eq!(calculate_priorities(TEST_INPUT), 70)
    }
}
