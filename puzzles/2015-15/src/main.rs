use advent_of_code::{Answer, Puzzle};
use puzzle_2015_15::part_1;

fn main() {
    Puzzle {
        title: "Science for Hungry People",
        year: 2015,
        day: 15,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the highest scoring cookie scores {answer}")
            })
        },
        part_2: |_input: &str| None,
    }
    .solve(include_str!("../input.txt"))
}
