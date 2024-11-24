use std::fmt::Display;
use std::time::{Duration, Instant};

pub struct Puzzle<Part1, Part2>
where
    Part1: FnOnce(&str) -> Solution,
    Part2: FnOnce(&str) -> Option<Solution>,
{
    pub name: &'static str,
    pub part_1: Part1,
    pub part_2: Part2,
}

impl<Part1, Part2> Puzzle<Part1, Part2>
where
    Part1: FnOnce(&str) -> Solution,
    Part2: FnOnce(&str) -> Option<Solution>,
{
    pub fn solve(self, input: &str) {
        println!("Solving puzzle: {}", self.name);

        let solution = (self.part_1)(input);
        println!(
            "Part 1: {} (took {}us)",
            solution.answer,
            solution.time.as_micros()
        );

        match (self.part_2)(input) {
            Some(solution) => {
                println!(
                    "Part 2: {} (took {}us)",
                    solution.answer,
                    solution.time.as_micros()
                );
            }
            None => {
                println!("Part 2: TODO")
            }
        }
    }
}

pub struct Solution {
    answer: String,
    time: Duration,
}

impl Solution {
    pub fn new<S, A, F>(solver: S, input: &str, formatter: F) -> Self
    where
        S: FnOnce(&str) -> A,
        A: Display,
        F: FnOnce(&str) -> String,
    {
        let time = Instant::now();
        let answer = solver(input);
        let time = time.elapsed();

        Self {
            answer: formatter(&answer.to_string()),
            time,
        }
    }
}
