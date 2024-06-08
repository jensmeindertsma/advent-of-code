use advent_of_code::{Answer, Puzzle};
use puzzle_2015_4::{part_1, part_2};

fn main() {
    Puzzle {
        title: "The Ideal Stocking Stuffer",
        year: 2015,
        day: 4,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("{answer} is the lowest positive number that produces a hash starting with five zeroes")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("{answer} is the lowest positive number that produces a hash starting with six zeroes")
            }))}
    }
    .solve(include_str!("../input.txt"))
}
