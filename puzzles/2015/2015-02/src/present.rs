pub struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    pub fn new(length: usize, width: usize, height: usize) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    pub fn sides(&self) -> [Side; 6] {
        let Self {
            length,
            width,
            height,
        } = *self;

        [
            Side::new(length, width),
            Side::new(length, height),
            Side::new(width, length),
            Side::new(width, height),
            Side::new(height, length),
            Side::new(height, width),
        ]
    }

    pub fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
}

pub struct Side {
    length: usize,
    width: usize,
}

impl Side {
    pub fn new(length: usize, width: usize) -> Self {
        Self { length, width }
    }

    pub fn area(&self) -> usize {
        self.length * self.width
    }

    pub fn perimeter(&self) -> usize {
        2 * self.length + 2 * self.width
    }
}
