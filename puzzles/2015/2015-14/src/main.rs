use ornament::{Puzzle, Solution};
use puzzle_2015_14::part_1;

fn main() {
    Puzzle {
        name: "Reindeer Olympics",
        year: 2015,
        day: 14,
        part_1: |input| {
            Solution::new(
                |input| part_1(input, 2503),
                input,
                |answer| format!("the winning reindeer traveled {answer} kilometers"),
            )
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
