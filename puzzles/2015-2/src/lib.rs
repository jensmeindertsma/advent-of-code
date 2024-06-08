use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::tuple,
};

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.parse()
                .expect("input lines should be valid dimension specifications!")
        })
        .map(|cuboid: Cuboid| {
            cuboid.surface_area()
                + cuboid
                    .faces()
                    .iter()
                    .map(|face| face.area())
                    .min()
                    .expect("there should be multiple faces to a cuboid")
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.parse()
                .expect("input lines should be valid dimension specifications!")
        })
        .map(|cuboid: Cuboid| {
            cuboid.volume()
                + cuboid
                    .faces()
                    .iter()
                    .map(|face| face.perimeter())
                    .min()
                    .expect("there should be multiple faces to a cuboid")
        })
        .sum()
}

struct Cuboid {
    length: usize,
    width: usize,
    height: usize,
}

impl FromStr for Cuboid {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        fn dimension(input: &str) -> nom::IResult<&str, usize> {
            map_res(digit1, |dimension: &str| dimension.parse())(input)
        }

        fn cuboid(input: &str) -> nom::IResult<&str, Cuboid> {
            map(
                tuple((dimension, tag("x"), dimension, tag("x"), dimension)),
                |(length, _, width, _, height)| Cuboid {
                    length,
                    width,
                    height,
                },
            )(input)
        }

        cuboid(string)
            .map(|(_, cuboid)| cuboid)
            .map_err(|_| "Invalid dimension specification!")
    }
}

impl Cuboid {
    pub fn faces(&self) -> [Face; 6] {
        [
            Face {
                length: self.length,
                width: self.width,
            },
            Face {
                length: self.length,
                width: self.width,
            },
            Face {
                length: self.width,
                width: self.height,
            },
            Face {
                length: self.width,
                width: self.height,
            },
            Face {
                length: self.height,
                width: self.length,
            },
            Face {
                length: self.height,
                width: self.length,
            },
        ]
    }

    pub fn surface_area(&self) -> usize {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    pub fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
}

struct Face {
    length: usize,
    width: usize,
}

impl Face {
    fn area(&self) -> usize {
        self.length * self.width
    }

    fn perimeter(&self) -> usize {
        2 * self.length + 2 * self.width
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("2x3x4"), 58);
        assert_eq!(part_1("1x1x10"), 43);

        assert_eq!(part_1(INPUT), 1588178);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2("2x3x4"), 34);
        assert_eq!(part_2("1x1x10"), 14);

        assert_eq!(part_2(INPUT), 3783758);
    }
}
