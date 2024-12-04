use ornament::{Puzzle, Solution};
use puzzle_2015_03::{part_1, part_2};

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
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("now {answer} houses receive at least one present")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
