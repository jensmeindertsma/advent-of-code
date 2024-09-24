use advent_of_code::{Answer, Puzzle};
use puzzle_2016_05::{part_1, part_2};

fn main() {
    Puzzle {
        title: "How About a Nice Game of Chess?",
        year: 2016,
        day: 5,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| format!("the password is {answer}"))
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the password is actually {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
