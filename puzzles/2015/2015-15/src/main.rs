use puzzle_2015_15::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Science for Hungry People",
        year: 2015,
        day: 15,
    }
    .part_one(part_one)
    .solve(INPUT)
}
