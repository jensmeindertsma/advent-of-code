use ornament::{Puzzle, Solution};
use puzzle_2015_19::part_1;

fn main() {
    Puzzle {
        name: "Medicine for Rudolph",
        year: 2015,
        day: 19,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("{answer} molecules can be created")
            })
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
