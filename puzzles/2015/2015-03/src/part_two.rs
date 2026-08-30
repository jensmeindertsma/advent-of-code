use std::collections::HashSet;

pub fn part_two(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();

    let mut santa_position = Position { x: 0, y: 0 };
    let mut robo_position = Position { x: 0, y: 0 };

    // Add starting position as visited house
    houses.insert(santa_position);

    for (turn, c) in input.trim().chars().enumerate() {
        let position = if turn % 2 == 0 {
            &mut santa_position
        } else {
            &mut robo_position
        };

        match c {
            '>' => position.x += 1,
            '<' => position.x -= 1,
            '^' => position.y += 1,
            'v' => position.y -= 1,
            _ => panic!("unexpected character `{c}`"),
        }

        houses.insert(*position);
    }

    houses.len()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}
