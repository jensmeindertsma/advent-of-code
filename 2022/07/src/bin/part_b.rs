use day_07::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let size = calculate_size(&input);

    println!("Total size of directories: {size}");
}

fn calculate_size(input: &str) -> usize {
    let commands = parse_input(input);
    let size_tree = generate_tree(&commands);

    println!("Size tree: {size_tree:#?}");

    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = size_tree.get(&vec!["/"]).unwrap();

    let free_space = total_size - used_space;
    let required_free = needed_space - free_space;

    let mut valid_directories: Vec<usize> = size_tree
        .iter()
        .filter(|(_, size)| **size > required_free)
        .map(|(_, size)| *size)
        .collect();

    valid_directories.sort();
    *valid_directories.first().unwrap()
}

#[cfg(test)]
mod tests {
    use day_07::TEST_INPUT;

    use super::calculate_size;

    #[test]
    fn test_input_a() {
        assert_eq!(calculate_size(TEST_INPUT), 24933642)
    }
}
