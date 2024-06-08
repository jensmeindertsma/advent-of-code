use advent_of_code::{Answer, Puzzle};
use puzzle_2015_10::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Elves Look, Elves Say",
        year: 2015,
        day: 10,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the length of the result is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the length of the new result is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
