use puzzle_template::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 13,
        day: 12,
        name: "The coolest puzzle template",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
