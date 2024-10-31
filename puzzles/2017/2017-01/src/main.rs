use advent_of_code::{Answer, Puzzle};
use puzzle_2017_01::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Inverse Captcha",
        year: 2017,
        day: 1,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the solution to the captcha is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the solution to the new captcha is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
