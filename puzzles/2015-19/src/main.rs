use advent_of_code::{Answer, Puzzle};
use puzzle_2015_19::part_1;

fn main() {
    Puzzle {
        title: "Medicine for Rudolph",
        year: 2015,
        day: 19,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("{answer} distinct molecules can be created")
            })
        },
        part_2: |_input: &str| None,
    }
    .solve(include_str!("../input.txt"))
}
