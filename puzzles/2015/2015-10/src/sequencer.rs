use itoa::Buffer as FormattingBuffer;
use std::{mem, ptr};

pub struct Sequencer {
    iterations: usize,
    formatter: FormattingBuffer,
    alpha: String,
    bravo: String,
}

impl Sequencer {
    pub fn new(input: &str, iterations: usize) -> Self {
        let mut alpha = String::from(input.trim());
        alpha.reserve(10_000_000);

        let bravo = String::with_capacity(10_000_000);

        Self {
            iterations,
            formatter: FormattingBuffer::new(),
            alpha,
            bravo,
        }
    }

    pub fn run(mut self) -> String {
        let mut current = &mut self.alpha;
        let mut next = &mut self.bravo;

        for _ in 0..self.iterations {
            let mut characters = current.chars().peekable();

            while let Some(character) = characters.next() {
                let mut count = 1;

                while let Some(&next_character) = characters.peek() {
                    if next_character == character {
                        count += 1;
                        characters.next();
                    } else {
                        break;
                    }
                }

                // After the whole "streak" has been counted, push the count
                // of the character followed by the character. For example:
                // - `21` -> `1211`, one two, one one
                // - `666344` -> `361324`

                next.push_str(self.formatter.format(count));
                next.push(character);
            }

            current.clear();

            mem::swap(&mut current, &mut next);
        }

        if ptr::eq(current, &self.alpha) {
            self.alpha
        } else {
            self.bravo
        }
    }
}
