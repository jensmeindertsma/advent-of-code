use ornament::{Puzzle, Solution};
use puzzle_2015_05::part_1;

fn main() {
    Puzzle {
        name: "Doesn't He Have Intern-Elves For This?",
        year: 2015,
        day: 5,
        part_1: |input| Solution::new(part_1, input, |answer| format!("{answer} strings are nice")),
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
