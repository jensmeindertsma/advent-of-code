mod banner;

use banner::Banner;
use comfy_table::{Attribute, Cell, Color, Table};
use owo_colors::OwoColorize;
use rand::seq::IndexedRandom;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub struct Puzzle {
    pub name: &'static str,
    pub year: u16,
    pub day: u8,
}

impl Puzzle {
    pub fn part_one<AnswerOne>(
        self,
        part_one: fn(&str) -> AnswerOne,
    ) -> PartialSolution<AnswerOne> {
        PartialSolution {
            puzzle: self,
            part_one,
        }
    }
}

pub struct PartialSolution<AnswerOne> {
    puzzle: Puzzle,
    part_one: fn(&str) -> AnswerOne,
}

impl<AnswerOne> PartialSolution<AnswerOne> {
    pub fn part_two<AnswerTwo>(
        self,
        part_two: fn(&str) -> AnswerTwo,
    ) -> CompleteSolution<AnswerOne, AnswerTwo> {
        CompleteSolution {
            puzzle: self.puzzle,
            part_one: self.part_one,
            part_two,
        }
    }
}

impl<AnswerOne> PartialSolution<AnswerOne>
where
    AnswerOne: Display,
{
    pub fn solve(&self, input: &str) {
        print_banner();
        print_message(&self.puzzle);

        let mut table = create_table();

        add_solution_row(Part::PartOne, &mut table, self.part_one, input);
        add_todo_row(&mut table);

        println!("{table}")
    }
}

pub struct CompleteSolution<AnswerOne, AnswerTwo> {
    puzzle: Puzzle,
    part_one: fn(&str) -> AnswerOne,
    part_two: fn(&str) -> AnswerTwo,
}

impl<AnswerOne, AnswerTwo> CompleteSolution<AnswerOne, AnswerTwo>
where
    AnswerOne: Display,
    AnswerTwo: Display,
{
    pub fn solve(&self, input: &str) {
        print_banner();
        print_message(&self.puzzle);

        let mut table = create_table();

        add_solution_row(Part::PartOne, &mut table, self.part_one, input);
        add_solution_row(Part::PartTwo, &mut table, self.part_two, input);

        println!("{table}")
    }
}

fn print_banner() {
    let options = [Banner::Snowman, Banner::Village];
    let mut rng = rand::rng();
    let banner = options
        .choose(&mut rng)
        .expect("the options list is not empty");

    println!("\n{banner}\n");
}

fn print_message(puzzle: &Puzzle) {
    println!(
        "{} {} {}{} {}\n",
        "==>".bold().red(),
        "solving puzzle".bold(),
        format!("{}-{:02}", puzzle.year, puzzle.day).bold().blue(),
        ":".bold(),
        format!("\"{}\"", puzzle.name).bold().italic()
    );
}

fn create_table() -> Table {
    let mut table = Table::new();

    table.load_preset("││──├─┼┤│─┼├┤┬┴┌┐└┘");

    table.set_header(vec![
        Cell::new("Puzzle").add_attribute(Attribute::Bold),
        Cell::new("Answer").add_attribute(Attribute::Bold),
        Cell::new("Time").add_attribute(Attribute::Bold),
    ]);

    table
}

enum Part {
    PartOne,
    PartTwo,
}

fn add_solution_row<Answer>(
    part: Part,
    table: &mut Table,
    part_one: fn(&str) -> Answer,
    input: &str,
) where
    Answer: Display,
{
    let now = Instant::now();

    let solution = part_one(input);

    let duration = now.elapsed();

    table.add_row(vec![
        Cell::new(match part {
            Part::PartOne => "Part 1",
            Part::PartTwo => "Part 2",
        })
        .add_attribute(Attribute::Bold),
        Cell::new(solution)
            .fg(Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new(format_time(duration)),
    ]);
}

fn add_todo_row(table: &mut Table) {
    table.add_row(vec![
        Cell::new("Part 2").add_attribute(Attribute::Bold),
        Cell::new("🚧")
            .fg(Color::Red)
            .add_attribute(Attribute::Bold),
        Cell::new("N/A"),
    ]);
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
