use advent_of_code::{Answer, Puzzle};
use puzzle_2015_13::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Knights of the Dinner Table",
        year: 2015,
        day: 13,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the total change in happiness is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the total change in happiness is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
