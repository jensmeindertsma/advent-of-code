use ornament::{Puzzle, Solution};
use puzzle_2015_01::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Not Quite Lisp",
        year: 2015,
        day: 1,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the instructions take Santa to floor {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("Santa first entered the basement at position {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
