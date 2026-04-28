use puzzle_2015_04::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Template",
        year: 2015,
        day: 4,
    }
    .part_one(part_one)
    .solve(INPUT)
}
