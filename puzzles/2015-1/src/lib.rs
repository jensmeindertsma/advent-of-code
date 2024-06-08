pub fn part_1(input: &str) -> isize {
    input
        .trim()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap())
        .fold(0, |floor, instruction| match instruction {
            Instruction::Ascend => floor + 1,
            Instruction::Descend => floor - 1,
        })
}

pub fn part_2(input: &str) -> usize {
    let instructions = input
        .trim()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap());

    let mut floor: isize = 0;

    for (index, instruction) in instructions.enumerate() {
        match instruction {
            Instruction::Ascend => floor += 1,
            Instruction::Descend => floor -= 1,
        }

        if floor.is_negative() {
            return index + 1;
        }
    }

    unreachable!()
}

enum Instruction {
    Ascend,
    Descend,
}

impl TryFrom<char> for Instruction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Ascend),
            ')' => Ok(Self::Descend),
            other => Err(other),
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);

        assert_eq!(part_1(INPUT), 138);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);

        assert_eq!(part_2(INPUT), 1771);
    }
}
