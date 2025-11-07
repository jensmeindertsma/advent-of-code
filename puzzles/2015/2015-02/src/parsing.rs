use crate::present::Present;
use std::{error::Error, fmt::Display, num::ParseIntError};

pub fn present(input: &str) -> Result<Present, ParseError> {
    let mut parts = input
        .trim()
        .split('x')
        .map(|x| x.parse().map_err(ParseError::Dimension));

    Ok(Present {
        length: parts.next().ok_or(ParseError::Presence("length"))??,
        width: parts.next().ok_or(ParseError::Presence("width"))??,
        height: parts.next().ok_or(ParseError::Presence("height"))??,
    })
}

#[derive(Debug)]
pub enum ParseError {
    Dimension(ParseIntError),
    Presence(&'static str),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dimension(parse_error) => write!(f, "failed to parse dimension: {parse_error}"),
            Self::Presence(dimension) => write!(f, "input missing `{dimension}` dimension"),
        }
    }
}

impl Error for ParseError {}
