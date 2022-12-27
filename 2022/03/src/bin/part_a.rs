use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let sum = calculate_priorities(&input);

    println!("Sum of priorities: {}", sum);
}

fn calculate_priorities(input: &str) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, character)| (character, index + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            assert_eq!(first.len(), second.len());

            let common_character = first
                .chars()
                .find(|character| second.contains(*character))
                .unwrap();

            letter_scores.get(&common_character).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day_03::TEST_INPUT;

    use super::calculate_priorities;

    #[test]
    fn example_list() {
        assert_eq!(calculate_priorities(TEST_INPUT), 157)
    }
}
