use nom::Finish;

pub struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    pub fn sides(&self) -> [(usize, usize); 6] {
        [
            (self.length, self.width),
            (self.length, self.width),
            (self.width, self.height),
            (self.width, self.height),
            (self.height, self.length),
            (self.height, self.length),
        ]
    }

    pub fn surface_area(&self) -> usize {
        self.sides().iter().map(|(x, y)| x * y).sum()
    }

    pub fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
}

impl TryFrom<&str> for Present {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parsing::present(value).finish() {
            Ok((_, present)) => Ok(present),
            Err(_) => Err(()),
        }
    }
}

mod parsing {
    use super::Present;
    use nom::{IResult, Parser, bytes, character, combinator};

    pub fn present(input: &str) -> IResult<&str, Present> {
        combinator::map(
            (
                dimension,
                bytes::complete::tag("x"),
                dimension,
                bytes::complete::tag("x"),
                dimension,
            ),
            |(length, _, width, _, height)| Present {
                length,
                width,
                height,
            },
        )
        .parse(input)
    }

    fn dimension(input: &str) -> IResult<&str, usize> {
        combinator::map_res(character::complete::digit1, |s: &str| s.parse()).parse(input)
    }
}
