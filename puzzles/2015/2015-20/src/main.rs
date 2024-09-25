use advent_of_code::{Answer, Puzzle};
use puzzle_2015_20::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Infinite Elves and Infinite Houses",
        year: 2015,
        day: 20,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the lowest house number is {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the new lowest house number is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
