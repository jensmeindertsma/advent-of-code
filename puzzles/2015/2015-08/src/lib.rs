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

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            line.encode().len() - line.len()
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

trait Encodeable {
    fn encode(&self) -> String;
}

impl Encodeable for &str {
    fn encode(&self) -> String {
        let mut string = self.to_string();
        string = string.replace('\\', "\\\\");
        format!("\"{}\"", string.replace('\"', "\\\""))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Encodeable, Measurable};

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!("\"\"".literal_length(), 2);
        assert_eq!("\"\"".memory_length(), 0);

        assert_eq!("\"abc\"".literal_length(), 5);
        assert_eq!("\"abc\"".memory_length(), 3);

        assert_eq!("\"aaa\\\"aaa\"".literal_length(), 10);
        assert_eq!("\"aaa\\\"aaa\"".memory_length(), 7);

        assert_eq!("\"\\x27\"".literal_length(), 6);
        assert_eq!("\"\\x27\"".memory_length(), 1);

        const EXAMPLE_INPUT: &str = "
            \"\"
            \"abc\"
            \"aaa\\\"aaa\"
            \"\\x27\"
        ";

        assert_eq!(
            EXAMPLE_INPUT
                .trim()
                .lines()
                .map(|line| line.trim().literal_length())
                .sum::<usize>(),
            23
        );
        assert_eq!(
            EXAMPLE_INPUT
                .trim()
                .lines()
                .map(|line| line.trim().memory_length())
                .sum::<usize>(),
            11
        );

        assert_eq!(part_1(EXAMPLE_INPUT), 12);

        assert_eq!(part_1(INPUT), 1350);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        println!("\"\\\"\\\"\"");
        assert_eq!("\"\"".encode(), "\"\\\"\\\"\"");
        assert_eq!("\"\"".len(), 2);
        assert_eq!("\"\"".encode().len(), 6);
        assert_eq!(part_2("\"\""), 6 - 2);

        assert_eq!("\"abc\"".encode(), "\"\\\"abc\\\"\"");
        assert_eq!("\"abc\"".len(), 5);
        assert_eq!("\"abc\"".encode().len(), 9);
        assert_eq!(part_2("\"abc\""), 9 - 5);

        assert_eq!("\"aaa\\\"aaa\"".encode(), "\"\\\"aaa\\\\\\\"aaa\\\"\"");
        assert_eq!("\"aaa\\\"aaa\"".len(), 10);
        assert_eq!("\"aaa\\\"aaa\"".encode().len(), 16);
        assert_eq!(part_2("\"aaa\\\"aaa\""), 16 - 10);

        assert_eq!("\"\\x27\"".encode(), "\"\\\"\\\\x27\\\"\"");
        assert_eq!("\"\\x27\"".len(), 6);
        assert_eq!("\"\\x27\"".encode().len(), 11);
        assert_eq!(part_2("\"\\x27\""), 11 - 6);

        assert_eq!(part_2(INPUT), 2085);
    }
}
