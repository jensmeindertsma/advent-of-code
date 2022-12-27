use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let count = count_fully_contained(&input);

    println!("Count: {count}");
}

fn count_fully_contained(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (first, second) = line.split_once(',').unwrap();

            let first_range = {
                let (start, end) = first.split_once('-').unwrap();
                let start: u16 = start.parse().unwrap();
                let end: u16 = end.parse().unwrap();

                start..=end
            };

            let second_range = {
                let (start, end) = second.split_once('-').unwrap();
                let start: u16 = start.parse().unwrap();
                let end: u16 = end.parse().unwrap();

                start..=end
            };

            let first_contained_in_second = first_range.contains(second_range.start())
                && first_range.contains(second_range.end());
            let second_contained_in_first = second_range.contains(first_range.start())
                && second_range.contains(first_range.end());

            first_contained_in_second || second_contained_in_first
        })
        .count()
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
