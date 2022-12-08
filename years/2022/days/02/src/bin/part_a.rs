use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let score = calculate_score(&input);

    println!("Total score: {}", score);
}

fn calculate_score(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use day_02::TEST_INPUT;

    use super::calculate_score;

    #[test]
    fn example_list() {
        assert_eq!(calculate_score(TEST_INPUT), 15)
    }
}
