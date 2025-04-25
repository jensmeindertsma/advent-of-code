use ornament::{Puzzle, Solution};
use puzzle_2015_20::part_1;

fn main() {
    Puzzle {
        name: "Infinite Elves and Infinite Houses",
        year: 2015,
        day: 20,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("{answer} is the lowest house number")
            })
        },
        part_2: None,
    }
    .solve(include_str!("../input.txt"))
}
