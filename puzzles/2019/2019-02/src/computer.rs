#[derive(Clone)]
pub struct Computer {
    program: Vec<usize>,
    instruction: usize,
}

impl Computer {
    pub fn with_program(source: &str) -> Self {
        let program = source
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            program,
            instruction: 0,
        }
    }

    pub fn set(&mut self, index: usize, value: usize) {
        self.program[index] = value
    }

    pub fn run(mut self) -> Output {
        loop {
            let opcode = self.program[self.instruction * 4];

            if opcode == 99 {
                return Output {
                    state: self.program,
                };
            }

            let location_1 = self.program[self.instruction * 4 + 1];
            let location_2 = self.program[self.instruction * 4 + 2];

            let value_1 = self.program[location_1];
            let value_2 = self.program[location_2];

            let output_value = match opcode {
                1 => value_1 + value_2,
                2 => value_1 * value_2,
                _ => panic!("unexpected opcode"),
            };

            let output_location = self.program[self.instruction * 4 + 3];
            self.program[output_location] = output_value;

            self.instruction += 1;
        }
    }
}

pub struct Output {
    pub state: Vec<usize>,
}
