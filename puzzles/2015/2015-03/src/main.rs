use ornament::{Puzzle, Solution};
use puzzle_2015_03::part_1;

fn main() {
    Puzzle {
        name: "Perfectly Spherical Houses in a Vacuum",
        year: 2015,
        day: 3,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("{answer} houses receive at least one present")
            })
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
