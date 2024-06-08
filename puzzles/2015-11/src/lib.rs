use itertools::Itertools;

pub fn part_1(input: &str) -> String {
    PasswordGenerator::from(input.trim())
        .find(|password| password.as_str().is_valid_password())
        .unwrap()
}

pub fn part_2(input: &str) -> String {
    PasswordGenerator::from(input.trim())
        .filter(|password| password.as_str().is_valid_password())
        .nth(1)
        .unwrap()
}

struct PasswordGenerator {
    previous: Vec<char>,
}

impl From<&str> for PasswordGenerator {
    fn from(value: &str) -> Self {
        Self {
            previous: value.chars().collect(),
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut increment_next = true;

        for c in self.previous.iter_mut().rev() {
            *c = if !increment_next {
                *c
            } else if *c == 'z' {
                increment_next = true;
                'a'
            } else {
                increment_next = false;
                ((*c as u8) + 1) as char
            };
        }

        Some(self.previous.iter().join(""))
    }
}

trait Password {
    fn is_valid_password(&self) -> bool;
}

impl Password for &str {
    fn is_valid_password(&self) -> bool {
        let three_ascending = self
            .chars()
            .tuple_windows()
            .any(|(a, b, c)| a as usize == b as usize - 1 && a as usize == c as usize - 2);

        let no_invalid = !self.chars().any(|c| matches!(c, 'i' | 'o' | 'l'));

        let two_pairs = self
            .chars()
            .tuple_windows()
            .enumerate()
            .any(|(index, (a, b))| {
                let index = index + 2;

                let remaining = if index <= self.len() {
                    &self[index..]
                } else {
                    ""
                };

                a == b && remaining.chars().tuple_windows().any(|(c, d)| c == d)
            });

        three_ascending && no_invalid && two_pairs
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::part_1;

        assert_eq!(part_1("abcdefgh"), "abcdffaa");
        assert_eq!(part_1("ghijklmn"), "ghjaabcc");

        assert_eq!(part_1(INPUT), "hepxxyzz");
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(INPUT), "heqaabcc")
    }
}
