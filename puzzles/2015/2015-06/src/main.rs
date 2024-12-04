use ornament::{Puzzle, Solution};
use puzzle_2015_06::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Probably a Fire Hazard",
        year: 2015,
        day: 6,
        part_1: |input| Solution::new(part_1, input, |answer| format!("{answer} lights are lit")),
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the total brightness is {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
