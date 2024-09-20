use std::ops::Deref;

pub fn part_1(input: &str) -> usize {
    Sequence::new(input.trim()).nth(39).unwrap().len()
}

pub fn part_2(input: &str) -> usize {
    Sequence::new(input.trim()).nth(49).unwrap().len()
}

struct Sequence {
    sentence: String,
}

impl Sequence {
    fn new(sentence: &str) -> Self {
        Self {
            sentence: sentence.to_string(),
        }
    }
}

impl Iterator for Sequence {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sets: Vec<String> = Vec::new();
        let mut characters = self.sentence.chars().peekable();

        while let Some(character) = characters.next() {
            let mut set = String::new();
            set.push(character);
            loop {
                if let Some(n) = characters.peek().cloned() {
                    if n == character {
                        characters.next();
                        set.push(n);
                        continue;
                    }
                }

                break;
            }
            sets.push(set)
        }

        self.sentence.clear();

        for set in sets {
            self.sentence
                .push_str(&format!("{}{}", set.len(), set.chars().next().unwrap()))
        }

        Some(self.sentence.clone())
    }
}

impl Deref for Sequence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.sentence
    }
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        use crate::{part_1, Sequence};

        let mut sequence = Sequence::new("1");

        assert_eq!(sequence.next().unwrap(), "11");
        assert_eq!(sequence.next().unwrap(), "21");
        assert_eq!(sequence.next().unwrap(), "1211");
        assert_eq!(sequence.next().unwrap(), "111221");
        assert_eq!(sequence.next().unwrap(), "312211");

        assert_eq!(part_1(INPUT), 329356);
    }

    #[test]
    fn part_2() {
        use crate::part_2;

        assert_eq!(part_2(INPUT), 4666278);
    }
}
