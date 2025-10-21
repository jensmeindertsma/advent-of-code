mod banner;

use banner::Banner;
use comfy_table::{Attribute, Cell, Color, Table};
use owo_colors::OwoColorize;
use rand::seq::IndexedRandom;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub struct Puzzle<One: Display, Two: Display> {
    pub year: u16,
    pub day: u8,
    pub name: &'static str,
    pub part_one: fn(&str) -> One,
    pub part_two: fn(&str) -> Two,
}

impl<One: Display, Two: Display> Puzzle<One, Two> {
    pub fn solve(&self, input: &str) {
        //Pick a banner randomly
        let options = [Banner::Snowman, Banner::Village];
        let mut rng = rand::rng();
        let banner = options
            .choose(&mut rng)
            .expect("the options list is not empty");

        println!("\n{banner}\n");

        println!(
            "{} {} {}{} {}\n",
            "==>".bold().red(),
            "solving puzzle".bold(),
            format!("{}-{:02}", self.year, self.day).bold().blue(),
            ":".bold(),
            format!("\"{}\"", self.name).bold().italic()
        );

        let mut table = Table::new();

        table.load_preset("││──├─┼┤│─┼├┤┬┴┌┐└┘");

        table.set_header(vec![
            Cell::new("Puzzle").add_attribute(Attribute::Bold),
            Cell::new("Answer").add_attribute(Attribute::Bold),
            Cell::new("Time").add_attribute(Attribute::Bold),
        ]);

        let now = Instant::now();
        let solution = (self.part_one)(input);
        let duration = now.elapsed();

        table.add_row(vec![
            Cell::new("Part 1").add_attribute(Attribute::Bold),
            Cell::new(solution)
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new(format_time(duration)),
        ]);

        let now = Instant::now();
        let solution = (self.part_two)(input);
        let duration = now.elapsed();

        table.add_row(vec![
            Cell::new("Part 2").add_attribute(Attribute::Bold),
            Cell::new(solution)
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new(format_time(duration)),
        ]);

        println!("{table}\n");
    }
}

fn format_time(time: Duration) -> String {
    let seconds = time.as_secs();

    if seconds > 0 {
        format!("{seconds}s")
    } else {
        let milliseconds = time.as_millis();

        if milliseconds > 0 {
            format!("{milliseconds}ms")
        } else {
            let microseconds = time.as_micros();
            format!("{microseconds}µs")
        }
    }
}
