use ornament::{Puzzle, Solution};
use puzzle_2015_11::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Corporate Policy",
        year: 2015,
        day: 11,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the next password is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the next password is {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
