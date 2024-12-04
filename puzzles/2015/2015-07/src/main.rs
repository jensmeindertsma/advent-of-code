use ornament::{Puzzle, Solution};
use puzzle_2015_07::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Some Assembly Required",
        year: 2015,
        day: 7,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("wire `a` has a signal of {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("wire `a` now has a signal of {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
