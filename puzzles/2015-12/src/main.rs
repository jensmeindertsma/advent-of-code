use advent_of_code::{Answer, Puzzle};
use puzzle_2015_12::{part_1, part_2};

fn main() {
    Puzzle {
        title: "JSAbacusFramework.io",
        year: 2015,
        day: 12,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the sum of all numbers in the document is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the sum of all numbers in the document is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
