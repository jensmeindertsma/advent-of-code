use crate::generator::PasswordGenerator;

pub fn part_2(input: &str) -> String {
    let mut generator = PasswordGenerator::new(input);

    // Ignore first password
    let _ = generator.generate();

    generator.generate()
}
