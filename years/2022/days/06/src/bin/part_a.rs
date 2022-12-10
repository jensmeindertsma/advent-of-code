use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let processed = count_processed(&input);

    println!("Processed: {processed}");
}

fn count_processed(input: &str) -> usize {
    todo!("{input}");
}

#[cfg(test)]
mod tests {
    use day_06::TEST_INPUT;

    use super::count_processed;

    #[test]
    fn example_list() {
        assert_eq!(count_processed(TEST_INPUT), 7)
    }
}
