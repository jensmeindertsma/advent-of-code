use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let start = (0, 0);
    let mut houses = HashSet::new();

    houses.insert(start);

    input
        .trim()
        .chars()
        .fold((houses, start), |(mut houses, (x, y)), character| {
            let new_location = match character {
                '>' => (x + 1, y),
                '<' => (x - 1, y),
                '^' => (x, y + 1),
                'v' => (x, y - 1),
                _ => panic!("Unexpected character `{character}`"),
            };

            houses.insert(new_location);

            (houses, new_location)
        })
        .0
        .len()
}
