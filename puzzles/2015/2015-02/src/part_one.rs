pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(parsing::present)
        .map(|present| present.expect("dimensions should always be valid"))
        .map(|present| {
            present.surface_area()
                + present
                    .sides()
                    .iter()
                    .map(|(x, y)| x * y)
                    .min()
                    .expect("a present should always have sides")
        })
        .sum()
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    pub fn sides(&self) -> [(u32, u32); 6] {
        [
            (self.length, self.width),
            (self.length, self.width),
            (self.width, self.height),
            (self.width, self.height),
            (self.height, self.length),
            (self.height, self.length),
        ]
    }

    pub fn surface_area(&self) -> u32 {
        self.sides().iter().map(|(x, y)| x * y).sum()
    }
}

mod parsing {
    use super::Present;
    use nom::{IResult, Parser, bytes, character, combinator};

    pub fn present(input: &str) -> Option<Present> {
        (
            dimension,
            bytes::complete::tag("x"),
            dimension,
            bytes::complete::tag("x"),
            dimension,
        )
            .parse(input)
            .map(|(_, (length, _, width, _, height))| Present {
                length,
                width,
                height,
            })
            .ok()
    }

    fn dimension(input: &str) -> IResult<&str, u32> {
        combinator::map_res(character::complete::digit1, |s: &str| s.parse()).parse(input)
    }
}
