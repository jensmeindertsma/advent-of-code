use crate::common::{PasswordGenerator, is_valid_password};

pub fn part_1(input: &str) -> String {
    PasswordGenerator::new(input.trim())
        .find(|p| is_valid_password(p))
        .unwrap()
}
