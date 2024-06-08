use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let starting_location = Location::new(0, 0);
    let mut visited_houses = HashSet::new();
    visited_houses.insert(starting_location.clone());

    input
        .trim()
        .chars()
        .map(|character| {
            Move::try_from(character).expect("all characters in input should be valid instructions")
        })
        .fold(
            (starting_location, visited_houses),
            |(mut location, mut visited_houses), direction| {
                match direction {
                    Move::East => location.x += 1,
                    Move::North => location.y += 1,
                    Move::South => location.y -= 1,
                    Move::West => location.x -= 1,
                };
                visited_houses.insert(location.clone());
                (location, visited_houses)
            },
        )
        .1
        .len()
}

pub fn part_2(input: &str) -> usize {
    let starting_location = Location::new(0, 0);
    let mut visited_houses = HashSet::new();
    visited_houses.insert(starting_location.clone());

    input
        .trim()
        .chars()
        .map(|character| {
            Move::try_from(character).expect("all characters in input should be valid instructions")
        })
        .enumerate()
        .fold(
            (starting_location.clone(), starting_location, visited_houses),
            |(mut santa_location, mut robo_location, mut visited_houses),
             (iteration, direction)| {
                let location = if iteration % 2 == 0 {
                    &mut santa_location
                } else {
                    &mut robo_location
                };

                match direction {
                    Move::East => location.x += 1,
                    Move::North => location.y += 1,
                    Move::South => location.y -= 1,
                    Move::West => location.x -= 1,
                };
                visited_houses.insert(location.clone());
                (santa_location, robo_location, visited_houses)
            },
        )
        .2
        .len()
}

enum Move {
    East,
    North,
    South,
    West,
}

impl TryFrom<char> for Move {
    type Error = char;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '>' => Ok(Self::East),
            '^' => Ok(Self::North),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            other => Err(other),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);

        assert_eq!(part_1(INPUT), 2572);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);

        assert_eq!(part_2(INPUT), 2631);
    }
}
