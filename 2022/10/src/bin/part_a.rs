use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let total = measure_signal_strength(&input);

    println!("Total signal strength: {total}");
}

fn measure_signal_strength(input: &str) -> i32 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let mut slices = line.split_whitespace();

            match slices.next().unwrap() {
                "noop" => Instruction::Sleep,
                "addx" => Instruction::Add(slices.next().unwrap().parse().unwrap()),
                other => panic!("Unexpected instruction: `{other}`"),
            }
        })
        .collect();

    let mut computer = Computer::new();

    for instruction in instructions {
        computer.execute(instruction);
    }

    computer
        .terminate()
        .iter()
        .filter(|snapshot| [20, 60, 100, 140, 180, 220].contains(&snapshot.cycle))
        .map(|snapshot| snapshot.cycle as i32 * snapshot.value)
        .sum()
}

#[derive(Debug)]
enum Instruction {
    Sleep,
    Add(i32),
}

struct Computer {
    current_cycle: u16,
    current_value: i32,
    snapshots: Vec<Snapshot>,
}

impl Computer {
    fn new() -> Self {
        Self {
            current_cycle: 0,
            current_value: 1,
            snapshots: Vec::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Sleep => {
                self.complete_cycle();
                self.snapshot();
            }
            Instruction::Add(amount) => {
                self.complete_cycle();
                self.snapshot();
                self.complete_cycle();
                self.snapshot();
                self.current_value += amount;
            }
        }
    }

    fn complete_cycle(&mut self) {
        self.current_cycle += 1
    }

    fn snapshot(&mut self) {
        self.snapshots.push(Snapshot {
            cycle: self.current_cycle,
            value: self.current_value,
        })
    }

    fn terminate(self) -> Vec<Snapshot> {
        self.snapshots
    }
}

struct Snapshot {
    cycle: u16,
    value: i32,
}

#[cfg(test)]
mod tests {
    use day_10::TEST_INPUT;

    use super::measure_signal_strength;

    #[test]
    fn test_input() {
        assert_eq!(measure_signal_strength(TEST_INPUT), 13140)
    }
}
