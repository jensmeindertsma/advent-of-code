use ornament::{Puzzle, Solution};
use puzzle_2015_16::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Aunt Sue",
        year: 2015,
        day: 16,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the number of Sue is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the number of Sue is {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
