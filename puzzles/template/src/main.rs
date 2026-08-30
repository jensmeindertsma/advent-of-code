use puzzle_template::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Template",
        year: 0,
        day: 0,
    }
    .part_one(part_one)
    .solve(INPUT)
}
