use nom::{Parser, branch::alt, multi::many0};

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            line.len() - calculate_memory_length(line)
        })
        .sum()
}

fn calculate_memory_length(input: &str) -> usize {
    use parsing::*;

    let string = input.strip_prefix('"').unwrap().strip_suffix('"').unwrap();

    let (_, numbers) = many0(alt((backslash, hexadecimal, double_quote, character)))
        .parse(string)
        .unwrap();

    numbers.iter().sum::<usize>()
}

mod parsing {
    use nom::{
        Parser,
        bytes::complete::{tag, take_while_m_n},
        character::complete::anychar,
        combinator::map,
    };

    pub fn backslash(input: &str) -> nom::IResult<&str, usize> {
        map(tag("\\\\"), |_| 1).parse(input)
    }

    pub fn hexadecimal(input: &str) -> nom::IResult<&str, usize> {
        map(
            (
                tag("\\x"),
                take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()),
            ),
            |_| 1,
        )
        .parse(input)
    }

    pub fn double_quote(input: &str) -> nom::IResult<&str, usize> {
        map(tag("\\\""), |_| 1).parse(input)
    }

    pub fn character(input: &str) -> nom::IResult<&str, usize> {
        map(anychar, |_| 1).parse(input)
    }
}
