use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, digit1},
    combinator::{map, map_res, peek},
    multi::many_till,
    sequence::{pair, tuple},
};
use std::{collections::HashMap, str::FromStr};

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|string| string.parse().unwrap())
        .filter(|room: &Room| room.has_valid_checksum())
        .map(|room| room.sector_id)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|string| string.parse().unwrap())
        .filter(|room: &Room| room.has_valid_checksum())
        .sorted_by(|a, b| b.sector_id.cmp(&a.sector_id))
        .map(|room: Room| (room.decrypted_name(), room.sector_id))
        .find(|(name, _)| name.contains("northpole"))
        .unwrap()
        .1
}

struct Room {
    encrypted_name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    fn has_valid_checksum(&self) -> bool {
        let mut letterbook = HashMap::new();

        for character in self.encrypted_name.chars().filter(|c| *c != '-') {
            *letterbook.entry(character).or_insert(0) += 1
        }

        let mut occurences = HashMap::new();

        for (letter, count) in letterbook {
            occurences.entry(count).or_insert(Vec::new()).push(letter);
        }

        // Now we just get an iterator over the keys, sort it, and then if there is just
        // one item in it list entry, we push it to our checksum. if there's more, we sort
        // those alphabetically, then push them all in order into our checksum.

        let mut generated_checksum = String::new();

        for letter in occurences
            .keys()
            .sorted()
            .rev()
            .flat_map(|p| occurences.get(p).unwrap().iter().sorted())
            .take(5)
        {
            generated_checksum.push(*letter);
        }

        generated_checksum == self.checksum
    }

    fn decrypted_name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let shift = c as usize - b'a' as usize + self.sector_id;
                    ((shift % 26) as u8 + b'a') as char
                }
            })
            .collect()
    }
}

impl FromStr for Room {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        fn room(input: &str) -> nom::IResult<&str, Room> {
            fn encrypted_name(input: &str) -> nom::IResult<&str, String> {
                map(
                    many_till(alt((tag("-"), alpha0)), peek(pair(tag("-"), digit1))),
                    |(parsed, _)| parsed.join(""),
                )(input)
            }

            fn sector_id(input: &str) -> nom::IResult<&str, usize> {
                map_res(digit1, |s: &str| s.parse())(input)
            }

            fn checksum(input: &str) -> nom::IResult<&str, &str> {
                map(tuple((tag("["), alpha1, tag("]"))), |(_, checksum, _)| {
                    checksum
                })(input)
            }

            map(
                tuple((encrypted_name, tag("-"), sector_id, checksum)),
                |(encrypted_name, _, sector_id, checksum)| Room {
                    encrypted_name: encrypted_name.to_string(),
                    sector_id,
                    checksum: checksum.to_owned(),
                },
            )(input)
        }

        room(string)
            .map(|parsed| parsed.1)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(
            part_1(
                "
                aaaaa-bbb-z-y-x-123[abxyz]
                a-b-c-d-e-f-g-h-987[abcde]
                not-a-real-room-404[oarel]
                totally-real-room-200[decoy]
                "
            ),
            1514
        );

        assert_eq!(part_1(INPUT), 409147);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(INPUT), 991);
    }
}
