use advent_of_code::{Answer, Puzzle};
use puzzle_2015_02::part_1;

fn main() {
    Puzzle {
        title: "I Was Told There Would Be No Math",
        year: 2015,
        day: 2,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the elves should order {answer} square feet of wrapping paper")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_1(input), |answer| {
                format!("the elves should order {answer} feet of ribbon")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
