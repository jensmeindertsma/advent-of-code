use advent_of_code::{Answer, Puzzle};
use puzzle_2015_19::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Medicine for Rudolph",
        year: 2015,
        day: 19,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("{answer} distinct molecules can be created")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the production takes at least {answer} steps")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
