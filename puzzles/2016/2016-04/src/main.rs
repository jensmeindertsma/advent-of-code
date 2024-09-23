use advent_of_code::{Answer, Puzzle};
use puzzle_2016_04::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Security Through Obscurity",
        year: 2016,
        day: 4,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the sum of the sector IDs of the real rooms is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the sector ID is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
