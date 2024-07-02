use advent_of_code::{Answer, Puzzle};
use puzzle_2015_07::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Some Assembly Required",
        year: 2015,
        day: 7,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("a signal of {answer} is ultimately provided to wire `a`")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("a new signal of {answer} is ultimately provided to wire `a`")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
