use crate::{generator::PasswordGenerator, validate::is_valid_password};

pub fn part_one(input: &str) -> String {
    PasswordGenerator::new(input.trim())
        .find(|password| is_valid_password(password))
        .expect("a valid password should exist")
}
