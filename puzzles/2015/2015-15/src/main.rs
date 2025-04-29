use ornament::{Puzzle, Solution};
use puzzle_2015_15::{part_1, part_2};

fn main() {
    Puzzle {
        name: "Science for Hungry People",
        year: 2015,
        day: 15,
        part_1: |input| {
            Solution::new(part_1, input, |answer| {
                format!("the total score of the highest-scoring cookie is {answer}")
            })
        },
        part_2: Some(|input| {
            Solution::new(part_2, input, |answer| {
                format!("the total score of the highest-scoring calorie-aware cookie is {answer}")
            })
        }),
    }
    .solve(include_str!("../input.txt"))
}
