use ornament::{Puzzle, Solution};
use puzzle_2016_01::part_1;

fn main() {
    Puzzle {
        name: "No Time for a Taxicab",
        year: 2016,
        day: 1,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("{answer} is the correct answer")
            })
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
