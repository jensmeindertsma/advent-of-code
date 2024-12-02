pub struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    pub fn from_dimensions(dimensions: &str) -> Self {
        let mut dimensions = dimensions.split('x');
        let length = dimensions.next().unwrap().parse().unwrap();
        let width = dimensions.next().unwrap().parse().unwrap();
        let height = dimensions.next().unwrap().parse().unwrap();

        Self {
            length,
            width,
            height,
        }
    }

    pub fn sides(&self) -> [Side; 6] {
        [
            Side::new(self.length, self.width),
            Side::new(self.length, self.height),
            Side::new(self.width, self.length),
            Side::new(self.width, self.height),
            Side::new(self.height, self.length),
            Side::new(self.height, self.width),
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
    fn new(length: usize, width: usize) -> Self {
        Self { length, width }
    }

    pub fn area(&self) -> usize {
        self.length * self.width
    }

    pub fn perimeter(&self) -> usize {
        2 * self.width + 2 * self.length
    }
}
