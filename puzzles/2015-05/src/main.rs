use advent_of_code::{Answer, Puzzle};
use puzzle_2015_05::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Doesn't He Have Intern-Elves For This?",
        year: 2015,
        day: 5,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| format!("{answer} strings are nice"))
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("under the new rules, {answer} strings are nice")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
