use advent_of_code::{Answer, Puzzle};
use puzzle_template::part_1;

fn main() {
    Puzzle {
        title: "Template",
        year: 9999,
        day: 99,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the input has a length of {answer}")
            })
        },
        part_2: |_input: &str| None,
    }
    .solve(include_str!("../input.txt"))
}
