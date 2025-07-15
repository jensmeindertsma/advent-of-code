use puzzle_template::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2000,
        day: 0,
        name: "Template",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
