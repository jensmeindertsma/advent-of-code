use itertools::Itertools;
use std::{collections::HashMap, str::Chars};

pub fn part_1(input: &str) -> String {
    let mut message = String::new();

    let mut lines: Vec<Chars<'_>> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars())
        .collect();

    let message_len = lines[0].clone().count();

    for _ in 0..message_len {
        let mut letter_occurences = HashMap::new();

        for char_iterator in lines.iter_mut() {
            let letter = char_iterator.next().unwrap();

            *letter_occurences.entry(letter).or_insert(0) += 1;
        }

        let most_frequent = letter_occurences
            .into_iter()
            .sorted_by(|(_, a), (_, b)| b.cmp(a))
            .map(|(c, _)| c)
            .next()
            .unwrap();
        message.push(most_frequent);
    }

    message
}

pub fn part_2(input: &str) -> String {
    let mut message = String::new();

    let mut lines: Vec<Chars<'_>> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars())
        .collect();

    let message_len = lines[0].clone().count();

    for _ in 0..message_len {
        let mut letter_occurences = HashMap::new();

        for char_iterator in lines.iter_mut() {
            let letter = char_iterator.next().unwrap();

            *letter_occurences.entry(letter).or_insert(0) += 1;
        }

        let most_frequent = letter_occurences
            .into_iter()
            // NOTE: this is the only line that is different from part 1.
            // We just revert the sorting ¯\_(ツ)_/¯
            .sorted_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(c, _)| c)
            .next()
            .unwrap();
        message.push(most_frequent);
    }

    message
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "
        eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar
    ";

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1(EXAMPLE_INPUT), "easter");

        assert_eq!(part_1(INPUT), "xhnqpqql");
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(EXAMPLE_INPUT), "advent");

        assert_eq!(part_2(INPUT), "brhailro");
    }
}
