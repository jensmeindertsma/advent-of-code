use ornament::{Puzzle, Solution};
use puzzle_2015_02::{part_1, part_2};

fn main() {
    Puzzle {
        name: "I Was Told There Would Be No Math",
        year: 2015,
        day: 2,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("{answer} square feet of wrapping paper should be ordered")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("{answer} feet of ribbon should be ordered")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
