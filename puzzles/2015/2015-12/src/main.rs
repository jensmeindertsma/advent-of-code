use ornament::{Puzzle, Solution};
use puzzle_2015_12::{part_1, part_2};

fn main() {
    Puzzle {
        name: "JSAbacusFramework.io",
        year: 2015,
        day: 12,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the sum of all numbers is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the sum of all numbers is now {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
