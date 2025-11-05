pub struct Present {
    pub length: usize,
    pub width: usize,
    pub height: usize,
}

impl Present {
    pub fn sides(&self) -> [Side; 6] {
        [
            Side {
                length: self.length,
                width: self.width,
            },
            Side {
                length: self.length,
                width: self.height,
            },
            Side {
                length: self.width,
                width: self.length,
            },
            Side {
                length: self.width,
                width: self.height,
            },
            Side {
                length: self.height,
                width: self.length,
            },
            Side {
                length: self.height,
                width: self.width,
            },
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
    pub fn area(&self) -> usize {
        self.length * self.width
    }

    pub fn perimeter(&self) -> usize {
        (self.length + self.width) * 2
    }
}
