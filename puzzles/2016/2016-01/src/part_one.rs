use crate::{direction::Direction, instruction::Instruction};

pub fn part_one(input: &str) -> usize {
    let mut direction = Direction::North;
    let mut x: isize = 0;
    let mut y: isize = 0;

    let (_, instructions) = parsing::document(input).unwrap();

    for instruction in instructions {
        let distance = match instruction {
            Instruction::Left(distance) => {
                direction = direction.left();
                distance
            }
            Instruction::Right(distance) => {
                direction = direction.right();
                distance
            }
        };

        match direction {
            Direction::East => x += distance as isize,
            Direction::North => y += distance as isize,
            Direction::South => y -= distance as isize,
            Direction::West => x -= distance as isize,
        }
    }

    (x.abs() + y.abs()) as usize
}

mod parsing {
    use crate::instruction::Instruction;
    use nom::{
        Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        multi::separated_list1,
    };

    pub fn document(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
        separated_list1(tag(", "), instruction).parse(input)
    }

    pub fn instruction(input: &str) -> nom::IResult<&str, Instruction> {
        map(
            (
                alt((tag("L"), tag("R"))),
                map_res(digit1, |n: &str| n.parse::<usize>()),
            ),
            |(turn, distance)| match turn {
                "L" => Instruction::Left(distance),
                "R" => Instruction::Right(distance),
                _ => unreachable!(),
            },
        )
        .parse(input)
    }
}
