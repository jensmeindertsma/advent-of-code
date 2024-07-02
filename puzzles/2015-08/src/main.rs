use advent_of_code::{Answer, Puzzle};
use puzzle_2015_08::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Matchsticks",
        year: 2015,
        day: 8,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the difference is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the difference is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
