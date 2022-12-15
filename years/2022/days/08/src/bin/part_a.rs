use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let count = count_visible_trees(&input);

    println!("Amount of visible trees: {count}");
}

fn count_visible_trees(input: &str) -> u32 {
    todo!("{input}");
}

#[cfg(test)]
mod tests {
    use day_08::TEST_INPUT;

    use super::count_visible_trees;

    #[test]
    fn test_input_a() {
        assert_eq!(count_visible_trees(TEST_INPUT), 21)
    }
}
