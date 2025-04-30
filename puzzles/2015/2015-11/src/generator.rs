use itertools::Itertools;

pub struct PasswordGenerator {
    current: Vec<char>,
}

impl PasswordGenerator {
    pub fn new(old: &str) -> Self {
        Self {
            current: old.chars().collect(),
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut increment_next = true;

        for c in self.current.iter_mut().rev() {
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

        Some(self.current.iter().join(""))
    }
}

pub fn is_valid_password(password: &str) -> bool {
    let three_ascending = password
        .chars()
        .tuple_windows()
        .any(|(a, b, c)| a as usize == b as usize - 1 && a as usize == c as usize - 2);

    let no_invalid = !password.chars().any(|c| matches!(c, 'i' | 'o' | 'l'));

    let two_pairs = password
        .chars()
        .tuple_windows()
        .enumerate()
        .any(|(index, (a, b))| {
            let index = index + 2;

            let remaining = if index <= password.len() {
                &password[index..]
            } else {
                ""
            };

            a == b && remaining.chars().tuple_windows().any(|(c, d)| c == d)
        });

    three_ascending && no_invalid && two_pairs
}
