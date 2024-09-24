use advent_of_code::{Answer, Puzzle};
use puzzle_2016_06::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Signals and Noise",
        year: 2016,
        day: 6,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the error-corrected message is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("Santa's message is actually {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
