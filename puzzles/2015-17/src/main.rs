use advent_of_code::{Answer, Puzzle};
use puzzle_2015_17::{part_1, part_2};

fn main() {
    Puzzle {
        title: "No Such Thing as Too Much",
        year: 2015,
        day: 17,
        part_1: |input: &str| {
            Answer::new(part_1(input, 150), |answer| {
                format!("there are {answer} combinations")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input, 150), |answer| {
                format!("{answer} combinations use a minimal amount of containers")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
