use puzzle_2020_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2020,
        day: 1,
        name: "Report Repair",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
