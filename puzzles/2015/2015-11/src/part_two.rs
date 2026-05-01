use crate::{generator::PasswordGenerator, validate::is_valid_password};

pub fn part_two(input: &str) -> String {
    PasswordGenerator::new(input.trim())
        .filter(|password| is_valid_password(password))
        .nth(1)
        .expect("a valid password should exist")
}
