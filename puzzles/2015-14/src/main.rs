use advent_of_code::{Answer, Puzzle};
use puzzle_2015_14::{part_1, part_2};

fn main() {
    Puzzle {
        title: "Reindeer Olympics",
        year: 2015,
        day: 14,
        part_1: |input: &str| {
            Answer::new(part_1(input, 2503), |answer| {
                format!("the winning reindeer traveled a distance of {answer}")
            })
        },
        part_2: |input: &str| {
            Some({
                Answer::new(part_2(input, 2503), |answer| {
                    format!("the winning reindeer scored {answer} points")
                })
            })
        },
    }
    .solve(include_str!("../input.txt"))
}
