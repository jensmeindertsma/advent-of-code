use advent_of_code::{Answer, Puzzle};
use puzzle_2015_11::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Corporate Policy",
        year: 2015,
        day: 11,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("Santa's next password should be {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("Santa's next password should be {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
