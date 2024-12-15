use ornament::{Puzzle, Solution};
use puzzle_2015_13::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Knights of the Dinner Table",
        year: 2015,
        day: 13,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the total change in happiness is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the total change in happiness is actually {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
