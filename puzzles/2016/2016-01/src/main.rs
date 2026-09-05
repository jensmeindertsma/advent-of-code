use puzzle_2016_01::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "No Time for a Taxicab",
        year: 2016,
        day: 1,
    }
    .part_one(part_one)
    .solve(INPUT)
}
