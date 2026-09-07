pub fn part_one(input: &str) -> usize {
    let (_, (x, y)) = input
        .trim()
        .split(", ")
        .map(|instruction| {
            let (direction, distance) = instruction.split_at(1);

            (
                match direction {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!(),
                },
                distance.parse::<u8>().unwrap(),
            )
        })
        .fold(
            (Heading::North, (0i32, 0i32)),
            |(heading, (mut x, mut y)), (direction, distance)| {
                let new_heading = heading.turn(direction);

                match new_heading {
                    Heading::North => y += distance as i32,
                    Heading::East => x += distance as i32,
                    Heading::South => y -= distance as i32,
                    Heading::West => x -= distance as i32,
                }

                (new_heading, (x, y))
            },
        );

    (x.abs() + y.abs()) as usize
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn(self, direction: Direction) -> Self {
        match direction {
            Direction::Left => match self {
                Heading::North => Heading::West,
                Heading::West => Heading::South,
                Heading::South => Heading::East,
                Heading::East => Heading::North,
            },
            Direction::Right => match self {
                Heading::North => Heading::East,
                Heading::East => Heading::South,
                Heading::South => Heading::West,
                Heading::West => Heading::North,
            },
        }
    }
}
