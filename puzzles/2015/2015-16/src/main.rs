use ornament::{Puzzle, Solution};
use puzzle_2015_14::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Aunt Sue",
        year: 2015,
        day: 16,
        part_1: |input| {
            Solution::new(
                |input| part_1(input, 2503),
                input,
                |answer| format!("the number of Sue is {answer}"),
            )
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
