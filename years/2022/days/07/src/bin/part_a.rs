use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let size = calculate_size(&input);

    println!("Total size of directories: {size}");
}

fn calculate_size(input: &str) -> usize {
    todo!("{input}");
}

#[cfg(test)]
mod tests {
    use day_07::TEST_INPUT;

    use super::calculate_size;

    #[test]
    fn test_input_a() {
        assert_eq!(calculate_size(TEST_INPUT), 95437)
    }
}
