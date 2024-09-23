use advent_of_code::{Answer, Puzzle};
use puzzle_2016_05::part_1;

fn main() {
    Puzzle {
        title: "How About a Nice Game of Chess?",
        year: 2016,
        day: 5,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| format!("the password is {answer}"))
        },
        part_2: |_input: &str| None,
    }
    .solve(include_str!("../input.txt"))
}
