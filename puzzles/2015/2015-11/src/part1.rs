use crate::generator::PasswordGenerator;

pub fn part_1(input: &str) -> String {
    PasswordGenerator::new(input).generate()
}
