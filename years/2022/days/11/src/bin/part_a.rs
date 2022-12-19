use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let monkey_business = measure_monkey_business(&input);

    println!("Level of monkey business after 20 rounds: {monkey_business}");
}

fn measure_monkey_business(input: &str) -> usize {
    todo!("{input}");
}

#[cfg(test)]
mod tests {
    use day_11::TEST_INPUT;

    use super::measure_monkey_business;

    #[test]
    fn test_input() {
        assert_eq!(measure_monkey_business(TEST_INPUT), 11605)
    }
}
