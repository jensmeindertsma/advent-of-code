use advent_of_code::{Answer, Puzzle};
use puzzle_2015_3::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Perfectly Spherical Houses in a Vacuum",
        year: 2015,
        day: 3,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("{answer} houses receive at least one present")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("{answer} houses receive at least one present")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
