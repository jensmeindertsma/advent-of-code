use ornament::{Puzzle, Solution};
use puzzle_2015_09::{part_1, part_2};

fn main() {
    Puzzle {
        name: "All in a Single Night",
        year: 2015,
        day: 9,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the distance of the shortest route is  {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the distance of the longest route is  {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
