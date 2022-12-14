use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let calories = calculate_calories(&input);

    println!("Highest total calorie count: {}", calories);
}

fn calculate_calories(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf_section| {
            elf_section
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use day_01::TEST_INPUT;

    use super::calculate_calories;

    #[test]
    fn example_list() {
        assert_eq!(calculate_calories(TEST_INPUT), 24000)
    }
}
