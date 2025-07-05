use comfy_table::{Attribute, Cell, Color, Table};
use indoc::indoc;
use owo_colors::OwoColorize;
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
    // TODO: make this a very pretty output with a table and such
    pub fn solve(&self, input: &str) {
        let banner = create_banner();

        println!("{banner}\n");

        println!(
            "{} {} {} ",
            "==>".bold().red(),
            "solving puzzle".bold(),
            format!("{}-{:02}", self.year, self.day).bold().blue()
        );

        let mut table = Table::new();

        table.load_preset("││──├─┼┤│─┼├┤┬┴┌┐└┘");

        table.set_header(vec![
            Cell::new("🎄").add_attribute(Attribute::Italic),
            Cell::new("Solution"),
            Cell::new("Time"),
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

        println!("{table}");
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

fn create_banner() -> String {
    let line_one = format!("     {}", "__".bold().red());
    let line_two = format!("   {}", "_|==|_".bold().red());
    let line_three = format!("    {}{}", "('')".bold().white(), "___/".bold().yellow());
    let line_four = format!("{}{}", ">--".bold().yellow(), "(`^^')".bold().white());
    let line_five = format!("  {}", "(`^'^'`)".bold().white());
    let line_six = format!("  {}", "`======' ".bold().white());

    format!("{line_one}\n{line_two}\n{line_three}\n{line_four}\n{line_five}\n{line_six}")
}
