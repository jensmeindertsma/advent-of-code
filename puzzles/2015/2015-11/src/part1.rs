use crate::common::{is_valid_password, PasswordGenerator};

pub fn part_1(input: &str) -> String {
    PasswordGenerator::new(input.trim())
        .find(|p| is_valid_password(p))
        .unwrap()
}
