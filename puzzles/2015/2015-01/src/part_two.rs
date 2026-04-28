pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .chars()
        .scan(0, |floor, c| {
            match c {
                '(' => *floor += 1,
                ')' => *floor -= 1,
                _ => panic!("unexpected character `{c}`"),
            }
            Some(*floor)
        })
        .position(|floor| floor < 0)
        .expect("a basement position should occur")
        + 1
}
