use std::collections::HashSet;

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .chars()
        .enumerate()
        .fold(
            ((0, 0), (0, 0), HashSet::from([(0, 0)])),
            |((mut santa_x, mut santa_y), (mut robo_x, mut robo_y), mut houses),
             (turn, direction)| {
                let (turn_x, turn_y) = if turn.is_multiple_of(2) {
                    (&mut santa_x, &mut santa_y)
                } else {
                    (&mut robo_x, &mut robo_y)
                };

                match direction {
                    '^' => *turn_y += 1,
                    'v' => *turn_y -= 1,
                    '>' => *turn_x += 1,
                    '<' => *turn_x -= 1,
                    other => panic!("unexpected character `{other}`"),
                };

                houses.insert((*turn_x, *turn_y));

                ((santa_x, santa_y), (robo_x, robo_y), houses)
            },
        )
        .2
        .len()
}
