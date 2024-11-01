use advent_of_code::{Answer, Puzzle};
use puzzle_2017_02::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Corruption Checksum",
        year: 2017,
        day: 2,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| format!("the checksum is {answer}"))
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the sum is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
