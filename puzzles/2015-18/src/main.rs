use advent_of_code::{Answer, Puzzle};
use puzzle_2015_18::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Like a GIF For Your Yard",
        year: 2015,
        day: 18,
        part_1: |input: &str| {
            Answer::new(part_1(input, 100), |answer| {
                format!("{answer} lights are on after 100 steps")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input, 100), |answer| {
                format!("{answer} lights are on after 100 steps")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
