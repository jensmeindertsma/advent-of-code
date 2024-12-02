use ornament::{Puzzle, Solution};
use puzzle_template::part_1;

fn main() {
    Puzzle {
        name: "I Was Told There Would Be No Math",
        year: 9999,
        day: 99,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("{answer} is the correct answer")
            })
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
