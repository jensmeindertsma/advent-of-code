pub fn part_one(input: &str) -> usize {
    input.trim().lines().map(|line| line.len()).sum()
}

struct Cube {
    length: u8,
    width: u8,
    height: u8,
}

impl Cube {
    fn new(length: u8, width: u8, height: u8) -> Self {
        Self {
            length,
            width,
            height,
        }
    }
}
