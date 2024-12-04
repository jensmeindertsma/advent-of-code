use comfy_table::{Attribute, Cell, Table};
use owo_colors::OwoColorize;
use std::fmt::Display;
use std::time::{Duration, Instant};

pub struct Puzzle {
    pub name: &'static str,
    pub year: usize,
    pub day: usize,
    pub part_1: fn(&str) -> Solution,
    pub part_2: Option<fn(&str) -> Solution>,
}

impl Puzzle {
    pub fn solve(self, input: &str) {
        let mut table = Table::new();
        table.load_preset("││──├─┼┤│─┼├┤┬┴┌┐└┘");

        table.set_header(vec![
            Cell::new(format!("{}-{:0>2}", self.year, self.day)).add_attribute(Attribute::Bold),
            Cell::new("Answer 🌟").add_attribute(Attribute::Bold),
            Cell::new("Time ⌛").add_attribute(Attribute::Bold),
        ]);

        let solution = (self.part_1)(input);

        table.add_row(vec![
            Cell::new("Part 1").add_attribute(Attribute::Bold),
            Cell::new(format!("\"{}\"", solution.answer)).add_attribute(Attribute::Italic),
            Cell::new(format_time(solution.time)),
        ]);

        match self.part_2 {
            Some(f) => {
                let solution = f(input);
                table.add_row(vec![
                    Cell::new("Part 2").add_attribute(Attribute::Bold),
                    Cell::new(format!("\"{}\"", solution.answer)).add_attribute(Attribute::Italic),
                    Cell::new(format_time(solution.time)),
                ]);
            }
            None => {
                table.add_row(vec![
                    Cell::new("Part 2").add_attribute(Attribute::Bold),
                    Cell::new("🎄 unsolved .?. 🎅"),
                    Cell::new("🎁"),
                ]);
            }
        }

        println!("{table}");
    }
}

fn format_time(time: Duration) -> String {
    if time.as_micros() < 1000 {
        format!("{}μs", time.as_micros())
    } else if time.as_millis() < 1000 {
        format!("{:.1}ms", time.as_micros() as f64 / 1000.0)
    } else {
        format!("{:.2}s", time.as_micros())
    }
}

pub struct Solution {
    answer: String,
    time: Duration,
}

impl Solution {
    pub fn new<Answer: Display>(
        solver: fn(&str) -> Answer,
        input: &str,
        formatter: fn(&str) -> String,
    ) -> Self {
        let time = Instant::now();
        let answer = solver(input);
        let time = time.elapsed();

        Self {
            time,
            answer: formatter(&format!("`{}`", answer.to_string().bold())),
        }
    }
}
