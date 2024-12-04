use ornament::{Puzzle, Solution};
use puzzle_2015_08::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Matchsticks",
        year: 2015,
        day: 8,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the number of characters is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the number of characters is {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
