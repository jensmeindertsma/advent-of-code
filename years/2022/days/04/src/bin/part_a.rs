use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let count = count_fully_contained(&input);

    println!("Count: {count}");
}

fn count_fully_contained(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use day_04::TEST_INPUT;

    use super::count_fully_contained;

    #[test]
    fn example_list() {
        assert_eq!(count_fully_contained(TEST_INPUT), 2)
    }
}
