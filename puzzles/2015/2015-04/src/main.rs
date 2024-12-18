use ornament::{Puzzle, Solution};
use puzzle_2015_04::{part_1, part_2};

fn main() {
    Puzzle {
        name: "The Ideal Stocking Stuffer",
        year: 2015,
        day: 4,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the lowest number is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the lowest number is {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
