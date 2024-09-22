use advent_of_code::{Answer, Puzzle};
use puzzle_2016_01::{part_1, part_2};

fn main() {
    Puzzle {
        title: "No Time for a Taxicab",
        year: 2016,
        day: 1,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("Easter Bunny HQ is {answer} blocks away")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("The first location visited twice is {answer} blocks away")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
