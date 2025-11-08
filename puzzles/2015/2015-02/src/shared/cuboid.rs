pub struct Cuboid {
    pub length: usize,
    pub width: usize,
    pub height: usize,
}

impl Cuboid {
    pub fn faces(&self) -> [(usize, usize); 6] {
        [
            (self.length, self.width),
            (self.length, self.height),
            (self.width, self.length),
            (self.width, self.height),
            (self.height, self.length),
            (self.height, self.width),
        ]
    }
}
