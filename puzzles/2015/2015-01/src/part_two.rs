pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .chars()
        .scan(0isize, |floor, character| {
            match character {
                '(' => *floor += 1,
                ')' => *floor -= 1,
                other => panic!("Unexpected character `{other}`"),
            };
            Some(*floor)
        })
        .enumerate()
        .find(|(_, floor)| floor.is_negative())
        .map(|(index, _)| index + 1)
        .expect("there should a negative character")
}
