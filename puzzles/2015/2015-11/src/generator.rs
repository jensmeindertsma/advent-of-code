use itertools::Itertools;
use std::str;

pub struct PasswordGenerator {
    current: Vec<char>,
}

impl PasswordGenerator {
    pub fn new(input: &str) -> Self {
        Self {
            current: input.trim().chars().collect(),
        }
    }

    fn next_password(&mut self) {
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
    }

    fn validate_password(&self) -> bool {
        if !self
            .current
            .iter()
            .tuple_windows()
            .any(|(a, b, c)| *a as usize == *b as usize - 1 && *a as usize == *c as usize - 2)
        {
            return false;
        };

        if self.current.iter().any(|c| matches!(c, 'i' | 'o' | 'l')) {
            return false;
        }

        if !self
            .current
            .iter()
            .tuple_windows()
            .enumerate()
            .any(|(index, (a, b))| {
                let index = index + 2;

                let remaining = if index <= self.current.len() {
                    &self.current[index..]
                } else {
                    &[]
                };

                a == b && remaining.iter().tuple_windows().any(|(c, d)| c == d)
            })
        {
            return false;
        }

        true
    }

    pub fn generate(&mut self) -> String {
        loop {
            self.next_password();

            if self.validate_password() {
                return self.current.iter().collect();
            }
        }
    }
}
