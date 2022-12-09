use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let count = count_overlapping(&input);

    println!("Count: {count}");
}

fn count_overlapping(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (left, right) = line.split_once(',').unwrap();

            let left = {
                let (start, end) = left.split_once('-').unwrap();
                let start: u16 = start.parse().unwrap();
                let end: u16 = end.parse().unwrap();

                start..=end
            };

            let right = {
                let (start, end) = right.split_once('-').unwrap();
                let start: u16 = start.parse().unwrap();
                let end: u16 = end.parse().unwrap();

                start..=end
            };

            left.contains(right.start())
                || left.contains(right.end())
                || right.contains(left.start())
                || right.contains(left.end())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use day_04::TEST_INPUT;

    use super::count_overlapping;

    #[test]
    fn example_list() {
        assert_eq!(count_overlapping(TEST_INPUT), 4)
    }
}
