use std::{fmt::Display, time::Instant};

pub struct Puzzle<One: Display, Two: Display> {
    pub part_one: fn(&str) -> One,
    pub part_two: Option<fn(&str) -> Two>,
}

impl<One: Display, Two: Display> Puzzle<One, Two> {
    pub fn solve(&self, input: &str) {
        let now = Instant::now();
        let solution_one = (self.part_one)(input);
        let duration = now.elapsed();

        println!(
            "part 1 = `{solution_one}` (took {}ms)",
            duration.as_millis()
        );

        if let Some(part_two) = self.part_two {
            let now = Instant::now();
            let solution_one = part_two(input);
            let duration = now.elapsed();

            println!(
                "part 2 = `{solution_one}` (took {}ms)",
                duration.as_millis()
            )
        } else {
            println!("part 2 unimplemented")
        }
    }
}
