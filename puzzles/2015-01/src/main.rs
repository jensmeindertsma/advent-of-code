use advent_of_code::{Answer, Puzzle};
use puzzle_2015_01::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Not Quite Lisp",
        year: 2015,
        day: 1,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the instructions take Santa to floor {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("character {answer} first takes Santa to the basement",)
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
