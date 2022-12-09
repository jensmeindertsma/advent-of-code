use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let sum = calculate_priorities(&input);

    println!("Sum of priorities: {}", sum);
}

fn calculate_priorities(_input: &str) -> u32 {
    todo!()
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
