use ornament::{Puzzle, Solution};
use puzzle_2015_10::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Elves Look, Elves Say",
        year: 2015,
        day: 10,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the length of the result is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the length of the new result is {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
