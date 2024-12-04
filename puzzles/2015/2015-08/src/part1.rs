use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::anychar,
    combinator::map,
    multi::many0,
    sequence::tuple,
};

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            line.literal_length() - line.memory_length()
        })
        .sum()
}

trait Measurable {
    fn literal_length(&self) -> usize;
    fn memory_length(&self) -> usize;
}

impl Measurable for &str {
    fn literal_length(&self) -> usize {
        self.len()
    }

    fn memory_length(&self) -> usize {
        fn backslash(input: &str) -> nom::IResult<&str, usize> {
            map(tag("\\\\"), |_| 1)(input)
        }

        fn hexadecimal(input: &str) -> nom::IResult<&str, usize> {
            map(
                tuple((
                    tag("\\x"),
                    take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()),
                )),
                |_| 1,
            )(input)
        }

        fn double_quote(input: &str) -> nom::IResult<&str, usize> {
            map(tag("\\\""), |_| 1)(input)
        }

        fn character(input: &str) -> nom::IResult<&str, usize> {
            map(anychar, |_| 1)(input)
        }

        let string = self.strip_prefix('"').unwrap().strip_suffix('"').unwrap();

        let (_, numbers) =
            many0(alt((backslash, hexadecimal, double_quote, character)))(string).unwrap();

        numbers.iter().sum::<usize>()
    }
}
