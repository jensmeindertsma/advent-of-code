use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();

    let mut position = Position { x: 0, y: 0 };

    // Add starting position as visited house
    houses.insert(position);

    for c in input.trim().chars() {
        match c {
            '>' => position.x += 1,
            '<' => position.x -= 1,
            '^' => position.y += 1,
            'v' => position.y -= 1,
            _ => panic!("unexpected character `{c}`"),
        }

        houses.insert(position);
    }

    houses.len()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}
