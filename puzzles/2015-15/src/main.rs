use advent_of_code::{Answer, Puzzle};
use puzzle_2015_15::{part_1, part_2};

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
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the highest scoring cookie with 500 calories scores {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
