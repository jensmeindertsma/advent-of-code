use std::collections::HashSet;

pub fn part_two(input: &str) -> usize {
    let start = (0, 0);
    let mut houses = HashSet::new();

    houses.insert(start);

    input
        .trim()
        .chars()
        .enumerate()
        .fold(
            (houses, start, start),
            |(mut houses, mut santa, mut robot), (iteration, character)| {
                let (x, y) = if iteration.is_multiple_of(2) {
                    &mut santa
                } else {
                    &mut robot
                };

                match character {
                    '>' => *x += 1,
                    '<' => *x -= 1,
                    '^' => *y += 1,
                    'v' => *y -= 1,
                    _ => panic!("Unexpected character `{character}`"),
                };

                houses.insert((*x, *y));

                (houses, santa, robot)
            },
        )
        .0
        .len()
}
