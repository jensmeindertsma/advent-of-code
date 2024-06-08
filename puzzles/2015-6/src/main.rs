use advent_of_code::{Answer, Puzzle};
use puzzle_2015_6::{part_1, part_2};

fn main() {
    println!(
        "{}",
        part_1(
            "turn on 226,196 through 599,390
    toggle 322,558 through 977,958
    turn off 660,55 through 986,197"
        )
    );

    Puzzle {
        title: "Probably a Fire Hazard",
        year: 2015,
        day: 6,
        part_1: |input: &str| {
            Answer::new(part_1(input), |answer| format!("{answer} lights are lit"))
        },
        part_2: |input: &str| {
            Some(Answer::new(part_2(input), |answer| {
                format!("the total brightness is {answer}")
            }))
        },
    }
    .solve(include_str!("../input.txt"))
}
