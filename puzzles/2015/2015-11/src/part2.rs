use crate::generator::{PasswordGenerator, is_valid_password};

pub fn part_2(input: &str) -> String {
    PasswordGenerator::new(input.trim())
        .filter(|p| is_valid_password(p))
        .nth(1)
        .unwrap()
}
