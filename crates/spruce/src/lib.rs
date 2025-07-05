use comfy_table::{Attribute, Cell, Color, Table};
use core::fmt::{self, Formatter};
use owo_colors::OwoColorize;
use rand::seq::IndexedRandom;
//use rand::seq::IndexedRandom;
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

        println!("{banner}\n");

        println!(
            "{} {} {}{} {}",
            "==>".bold().red(),
            "solving puzzle".bold(),
            format!("{}-{:02}", self.year, self.day).bold().blue(),
            ":".bold(),
            format!("\"{}\"", self.name).bold().italic()
        );

        let mut table = Table::new();

        table.load_preset("││──├─┼┤│─┼├┤┬┴┌┐└┘");

        table.set_header(vec![
            Cell::new("  🎄  "),
            Cell::new("Solution").add_attribute(Attribute::Bold),
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

enum Banner {
    Snowman,
    Village,
}

impl fmt::Display for Banner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Snowman => {
                let line_one = format!("     {}", "__".bold().red());
                let line_two = format!("   {}", "_|==|_".bold().red());
                let line_three = format!("    {}{}", "('')".bold().white(), "___/".bold().yellow());
                let line_four = format!("{}{}", ">--".bold().yellow(), "(`^^')".bold().white());
                let line_five = format!("  {}", "(`^'^'`)".bold().white());
                let line_six = format!("  {}", "`======' ".bold().white());

                write!(
                    f,
                    "{line_one}\n{line_two}\n{line_three}\n{line_four}\n{line_five}\n{line_six}"
                )
            }

            Self::Village => {
                let line_one = format!(
                    "   {}                                                   {}",
                    ".-.".bold().yellow(),
                    "\\ /".bold().yellow()
                );
                let line_two = format!(
                    "  {}                                {}                  {} {} {}",
                    "( (".bold().yellow(),
                    "|".bold().black(),
                    "-".bold().yellow(),
                    "*".bold().red(),
                    "-".bold().yellow()
                );
                let line_three = format!(
                    "   {}                              {}                  {}",
                    "'-`".bold().yellow(),
                    "-+-".bold().black(),
                    "/ \\".bold().yellow()
                );
                let line_four = format!(
                    "            {}            {}          {}{}{}          {}",
                    "\\".bold().black(),
                    "o".bold().yellow(),
                    "_".bold().red(),
                    "|".bold().black(),
                    "_".bold().red(),
                    "\\".bold().black()
                );
                let line_five = format!(
                    "            {}          {}        {}         {}",
                    "))".bold().black(),
                    "{^}".bold().green(),
                    "/___\\".bold().red(),
                    "))".bold().black()
                );
                let line_six = format!(
                    "          {}{}{}     {}     {}    {}{}{}",
                    ".-".bold().red(),
                    "#".bold().black(),
                    "-----.".bold().red(),
                    "/|\\".bold().green(),
                    ".---'-'---.".bold().red(),
                    ".-".bold().red(),
                    "#".bold().black(),
                    "-----.".bold().red()
                );
                let line_seven = format!(
                    "     {}   {}   {}  {}",
                    "___ /_________\\".bold().red(),
                    "//|\\\\".bold().green(),
                    "/___________\\".bold().red(),
                    "/_________\\".bold().red()
                );
                let line_eight = format!(
                    "    {} {}{} {} {}{}    {}    {} {} {} {} {}    {} {}{}{} {} {}",
                    "/___\\".bold().red(),
                    "|".bold().yellow(),
                    "[]".bold().blue(),
                    "_".bold().yellow(),
                    "[]".bold().blue(),
                    "|".bold().yellow(),
                    "//|\\\\".bold().green(),
                    "|".bold().yellow(),
                    "A".bold().blue(),
                    "/^\\".bold().yellow(),
                    "A".bold().blue(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "[]".bold().blue(),
                    "_".bold().yellow(),
                    "[]".bold().blue(),
                    "|".bold().yellow(),
                    "_.O,_".bold().white()
                );
                let line_nine = format!(
                    "{}{}{}{}{}{}{}{}  {}{}{}  {}{}{}{}{}   {}{}{}   {}{}{}  {}{}{}  {}{}",
                    "....".bold().white(),
                    "|".bold().yellow(),
                    "\"".bold().blue(),
                    "#".bold().black(),
                    "\"".bold().blue(),
                    "|".bold().yellow(),
                    ".".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "*".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "...".bold().white(),
                    "///|\\\\\\".bold().green(),
                    "...".bold().white(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "\"".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "....".bold().white(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "*".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "..(^)....".bold().white()
                );

                write!(
                    f,
                    "{line_one}\n{line_two}\n{line_three}\n{line_four}\n{line_five}\n{line_six}\n{line_seven}\n{line_eight}\n{line_nine}"
                )
            }
        }
    }
}
