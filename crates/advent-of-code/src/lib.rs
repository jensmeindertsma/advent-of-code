use core::fmt;
use owo_colors::OwoColorize;
use std::time::{Duration, Instant};

pub struct Puzzle<Part1, Part2> {
    pub title: &'static str,
    pub year: usize,
    pub day: usize,
    pub part_1: Part1,
    pub part_2: Part2,
}

impl<Part1, Part2> Puzzle<Part1, Part2>
where
    Part1: FnOnce(&str) -> Answer,
    Part2: FnOnce(&str) -> Option<Answer>,
{
    pub fn solve(self, input: &str) {
        // Calculate length of header, before formatting it. If we did this later, the number
        // would turn out way too high because of the length the escape codes add to the string.
        // The emoji takes 2 characters in width but the UTF-8 character has a length of 4, so we
        // replace it with two characters.
        let header_len = format!(
            "xx Advent of Code {} Day {} \"{}\" xx",
            self.year, self.day, self.title,
        )
        .len();

        println!("{}", "=".repeat(header_len).bold());
        println!(
            "🎄 {} {} {} {} {}{}{} 🎄",
            "Advent of Code".bold(),
            self.year.bold(),
            "Day".bold(),
            self.day.bold(),
            "\"".bold(),
            self.title.italic(),
            "\"".bold()
        );
        println!("{}", "=".repeat(header_len).bold());

        let now = Instant::now();
        let answer = (self.part_1)(input);
        let elapsed = now.elapsed();

        println!(
            "{}{} {} ({})",
            "Part 1".bold().green(),
            ":".bold(),
            answer,
            format_time(elapsed).italic()
        );

        let now = Instant::now();
        let answer = (self.part_2)(input);
        let elapsed = now.elapsed();

        println!(
            "{}{} {}",
            "Part 2".bold().green(),
            ":".bold(),
            match answer {
                None => "todo!".bold().italic().to_string(),
                Some(answer) => format!("{} ({})", answer, format_time(elapsed).italic()),
            }
        );
        println!("{}", "=".repeat(header_len).bold());
    }
}

pub struct Answer(String);

impl Answer {
    pub fn new<V, F>(value: V, formatter: F) -> Self
    where
        V: fmt::Display,
        F: FnOnce(&str) -> String,
    {
        let answer = format!(
            "{}{}{}{}",
            "`".bold(),
            value.bold().red(),
            "`".bold(),
            "".italic(),
        );
        Self(formatter(&answer))
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn format_time(duration: Duration) -> String {
    if duration.as_micros() < 1000 {
        format!("{}μs", duration.as_micros())
    } else if duration.as_millis() < 1000 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{:.2}s", duration.as_secs_f32())
    }
}
