pub fn part_one(input: &str) -> isize {
    input
        .trim()
        .chars()
        .fold(0, |floor, character| match character {
            '(' => floor + 1,
            ')' => floor - 1,
            other => panic!("Unexpected character `{other}`"),
        })
}
