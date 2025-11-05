use crate::present::Present;
use nom::{Parser, bytes::complete::tag, character::complete::digit1, combinator::map_res};

pub fn present(input: &str) -> nom::IResult<&str, Present> {
    fn dimension(input: &str) -> nom::IResult<&str, usize> {
        map_res(digit1, |n: &str| n.parse()).parse(input)
    }

    let (input, (length, _, width, _, height)) =
        (dimension, tag("x"), dimension, tag("x"), dimension).parse(input)?;

    Ok((
        input,
        Present {
            length,
            width,
            height,
        },
    ))
}
