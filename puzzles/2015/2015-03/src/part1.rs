use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .chars()
        .fold(
            ((0, 0), HashSet::from([(0, 0)])),
            |((x, y), mut houses), direction| {
                let next_house = match direction {
                    '^' => (x, y + 1),
                    'v' => (x, y - 1),
                    '>' => (x + 1, y),
                    '<' => (x - 1, y),
                    other => panic!("unexpected character `{other}`"),
                };

                houses.insert(next_house);

                (next_house, houses)
            },
        )
        .1
        .len()
}
