use ornament::{Puzzle, Solution};
use puzzle_2015_17::{part_1, part_2};

fn main() {
    Puzzle {
        name: "No Such Thing as Too Much",
        year: 2015,
        day: 17,
        part_1: |input| {
            Solution::new(
                |input| part_1(input, 150),
                input,
                |answer| format!("there are {answer} correct combintions"),
            )
        },
        part_2: Some(|input| {
            Solution::new(
                |input| part_2(input, 150),
                input,
                |answer| format!("there are {answer} correct combintions"),
            )
        }),
    }
    .solve(include_str!("../input.txt"))
}
