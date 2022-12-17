use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let sum = measure_signal_strength(&input);

    println!("Sum of signal strength measurements: {sum}");
}

fn measure_signal_strength(input: &str) -> usize {
    todo!("{input}");
}

#[cfg(test)]
mod tests {
    use day_10::TEST_INPUT;

    use super::measure_signal_strength;

    #[test]
    fn test_input() {
        assert_eq!(measure_signal_strength(TEST_INPUT), 13140)
    }
}
