pub fn part_one(input: &str) -> i32 {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("unexpected character `{c}`"),
        })
        .sum()
}
