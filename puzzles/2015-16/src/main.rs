use advent_of_code::{Answer, Puzzle};
use puzzle_2015_16::part_1;

fn main() {
    Puzzle {
        title: "Aunt Sue",
        year: 2015,
        day: 16,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| format!("Sue's number is {answer}"))
        },
        part_2: |_input: &str| None,
    }
    .solve(include_str!("../input.txt"))
}
