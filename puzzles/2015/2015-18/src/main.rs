use ornament::{Puzzle, Solution};
use puzzle_2015_18::part_1;

fn main() {
    Puzzle {
        name: "Like a GIF For Your Yard",
        year: 9999,
        day: 99,
        part_1: |input| {
            Solution::new(
                |input| part_1(input, 100),
                input,
                |answer| format!("{answer} lights are on"),
            )
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
