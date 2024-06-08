use advent_of_code::{Answer, Puzzle};
use puzzle_2015_14::part_1;

fn main() {
    Puzzle {
        title: "Reindeer Olympics",
        year: 2015,
        day: 14,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the winning reindeer traveled a distance of {answer}")
            })
        },
        part_2: |_input: &str| None,
    }
    .solve(include_str!("../input.txt"))
}
