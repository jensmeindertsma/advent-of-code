use advent_of_code::{Answer, Puzzle};
use puzzle_2015_9::{part_1, part_2};

fn main() {
    Puzzle {
        title: "All in a Single Night",
        year: 2015,
        day: 9,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| {
                format!("the distance of the shortest route is  {answer}")
            })
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the distance of the longest route is  {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
