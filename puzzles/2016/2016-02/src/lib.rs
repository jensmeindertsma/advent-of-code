use core::fmt::{self, Formatter};

pub fn part_1(input: &str) -> String {
    let keypad: [[u8; 3]; 3] = [
        // three-dimensional keypad
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    let mut code = String::new();

    // Start at the `5` button.
    let mut current_location = Location { x: 1, y: 1 };

    for button_instructions in input.trim().lines().map(|line| line.trim()).map(|line| {
        line.chars()
            .map(|char| Instruction::try_from(char).unwrap())
            .collect::<Vec<Instruction>>()
    }) {
        for instruction in button_instructions {
            let new_location = match instruction {
                Instruction::Down => {
                    let mut new_location = current_location;
                    new_location.y += 1;
                    new_location
                }
                Instruction::Left => {
                    let mut new_location = current_location;
                    new_location.x -= 1;
                    new_location
                }
                Instruction::Right => {
                    let mut new_location = current_location;
                    new_location.x += 1;
                    new_location
                }
                Instruction::Up => {
                    let mut new_location = current_location;
                    new_location.y -= 1;
                    new_location
                }
            };

            if let Some(row) = keypad.get(new_location.y) {
                if row.get(new_location.x).is_some() {
                    // We've checked we're still inside the bounds of the keypad
                    // So this means our new location is valid and is now current.
                    current_location = new_location;
                    continue;
                }
            }

            // If either x or y place us outside of the keypad, we ignore
            // the instruction and so our current location is unchanged.
        }

        code.push_str(&keypad[current_location.y][current_location.x].to_string());
    }

    code
}

pub fn part_2(input: &str) -> String {
    #[rustfmt::skip]
    let keypad = [
        [None,      None,       Some('1'),  None,       None],
        [None,      Some('2'),  Some('3'),  Some('4'),  None],
        [Some('5'), Some('6'),  Some('7'),  Some('8'),  Some('9')],
        [None,      Some('A'),  Some('B'),  Some('C'),  None],
        [None,      None,       Some('D'),  None,       None],
    ];
    let mut code = String::new();

    // Start at the `5` button.
    let mut current_location = Location { x: 0, y: 2 };

    for button_instructions in input.trim().lines().map(|line| line.trim()).map(|line| {
        line.chars()
            .map(|char| Instruction::try_from(char).unwrap())
            .collect::<Vec<Instruction>>()
    }) {
        for instruction in button_instructions {
            let new_location = match instruction {
                Instruction::Down => {
                    let mut new_location = current_location;
                    new_location.y += 1;
                    new_location
                }
                Instruction::Left => {
                    let mut new_location = current_location;
                    new_location.x -= 1;
                    new_location
                }
                Instruction::Right => {
                    let mut new_location = current_location;
                    new_location.x += 1;
                    new_location
                }
                Instruction::Up => {
                    let mut new_location = current_location;
                    new_location.y -= 1;
                    new_location
                }
            };

            if let Some(row) = keypad.get(new_location.y) {
                if let Some(option) = row.get(new_location.x) {
                    if option.is_some() {
                        // We've checked we're still inside the bounds of the keypad
                        // So this means our new location is valid and is now current.
                        current_location = new_location;
                        continue;
                    }
                }
            }

            // If either x or y place us outside of the keypad, we ignore
            // the instruction and so our current location is unchanged.
        }

        code.push(
            keypad[current_location.y][current_location.x]
                .expect("current location should be valid"),
        );
    }

    code
}

#[derive(Debug)]
enum Instruction {
    Down,
    Left,
    Right,
    Up,
}

impl TryFrom<char> for Instruction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            'U' => Self::Up,
            other => return Err(other),
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Location {
    x: usize,
    y: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(
            part_1(
                "
                ULL
                RRDDD
                LURDL
                UUUUD
                "
            ),
            "1985"
        );

        assert_eq!(part_1(INPUT), "56855");
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(
            part_2(
                "
                ULL
                RRDDD
                LURDL
                UUUUD
                "
            ),
            "5DB3"
        );

        //assert_eq!(part_1(INPUT), "56855");
    }
}
