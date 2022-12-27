use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let processed = count_processed(&input);

    println!("Processed: {processed}");
}

fn count_processed(input: &str) -> usize {
    let characters: Vec<char> = input.chars().collect();

    let (first_index, _) = characters
        .windows(14)
        .enumerate()
        .find(|(_, window)| {
            let mut map = HashSet::new();

            // `map.insert` returns `false` for duplicates, which we then negate to true.
            let contains_duplicates = window.iter().any(|char| !map.insert(*char));

            !contains_duplicates
        })
        .unwrap();

    first_index + 14
}

#[cfg(test)]
mod tests {
    use day_06::*;

    use super::count_processed;

    #[test]
    fn test_input_a() {
        assert_eq!(count_processed(TEST_INPUT_A), 19)
    }

    #[test]
    fn test_input_b() {
        assert_eq!(count_processed(TEST_INPUT_B), 23)
    }

    #[test]
    fn test_input_c() {
        assert_eq!(count_processed(TEST_INPUT_C), 23)
    }

    #[test]
    fn test_input_d() {
        assert_eq!(count_processed(TEST_INPUT_D), 29)
    }

    #[test]
    fn test_input_e() {
        assert_eq!(count_processed(TEST_INPUT_E), 26)
    }
}
