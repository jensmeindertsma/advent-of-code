#![feature(get_many_mut, iter_intersperse)]

use std::{fmt, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let output = process(&input);

    println!("{output}");
}

fn process(input: &str) -> String {
    let instructions: Vec<Instruction> = input.lines().map(Instruction::from).collect();

    let mut screen = String::with_capacity(240);
    let mut machine = Machine::new(&mut screen);

    for instruction in instructions {
        machine.process(&instruction);
    }

    screen
}

enum Instruction {
    Sleep,
    Add(i32),
}

impl Instruction {
    fn duration(&self) -> u8 {
        match self {
            Self::Sleep => 1,
            Self::Add(_) => 2,
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut slices = value.split_whitespace();

        match slices.next().unwrap() {
            "noop" => Self::Sleep,
            "addx" => Self::Add(slices.next().unwrap().parse().unwrap()),
            other => panic!("Unexpected instruction: {other}"),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::Sleep => "noop".to_string(),
                Self::Add(amount) => format!("addx {amount}"),
            }
        )
    }
}

struct Machine<'a> {
    current_cycle: usize,
    register: isize,
    screen: &'a mut String,
}

impl<'a> Machine<'a> {
    fn new(screen: &'a mut String) -> Self {
        Self {
            current_cycle: 1,
            register: 1,
            screen,
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.duration() {
            let sprite_position = (self.register - 1)..=(self.register + 1);
            let draw_position = self.current_cycle - 1;
            let lit = sprite_position.contains(&((draw_position % 40) as isize));

            if lit {
                self.screen.push('#')
            } else {
                self.screen.push('.')
            }

            if self.current_cycle >= 40 && self.current_cycle % 40 == 0 && self.current_cycle != 240
            {
                self.screen.push('\n')
            }

            self.current_cycle += 1
        }

        if let Instruction::Add(value) = instruction {
            self.register += *value as isize;
        }
    }
}

// impl<'a> From<&'a [Instruction]> for Processor<'a> {
//     fn from(instructions: &'a [Instruction]) -> Self {
//         Self {
//             instructions,
//             current_cycle: 1,
//             value: 1,
//             buffer: Vec::new(),
//         }
//     }
// }

// impl<'a> Processor<'a> {
//     fn run(mut self) -> String {
//         while let Some(instruction) = self.instructions.get(self.current_cycle - 1) {
//             match instruction {
//                 Instruction::Sleep => {
//                     self.draw();
//                     self.current_cycle += 1;
//                 }
//                 Instruction::Add(amount) => {}
//             }
//         }

//         self.buffer
//             .chunks(40)
//             .map(|chunk| chunk.iter().collect())
//             .intersperse('\n'.to_string())
//             .collect()
//     }

//     fn draw(&mut self) {
//         let draw_position = self.current_cycle - 1;
//         let sprite_position = (self.value - 1)..=(self.value + 1);
//         let lit = sprite_position.contains(&draw_position);

//         let mut display = vec!['.'; 40];
//         for position in sprite_position {
//             display[position] = '#'
//         }
//         if display[draw_position] == '#' {
//             display[draw_position] = 'D';
//         } else {
//             display[draw_position] = 'd'
//         }

//         println!("--- CYCLE {} --- LIT: {}", self.current_cycle, lit);
//         println!("{}", display.iter().collect::<String>());

//         if lit {
//             self.buffer.push('#')
//         } else {
//             self.buffer.push('.')
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use day_10::TEST_INPUT;

    use super::process;

    #[test]
    fn test_input() {
        assert_eq!(
            process(TEST_INPUT),
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######....."
        )
    }
}
