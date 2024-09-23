use advent_of_code::{Answer, Puzzle};
use puzzle_2016_03::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Squares With Three Sides",
        year: 2016,
        day: 3,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("{answer} of the listed triangles are possible")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("{answer} of the listed triangles are possible")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
