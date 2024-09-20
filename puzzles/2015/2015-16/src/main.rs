use advent_of_code::{Answer, Puzzle};
use puzzle_2015_16::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Aunt Sue",
        year: 2015,
        day: 16,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| format!("Sue's number is {answer}"))
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("Sue's real number is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
