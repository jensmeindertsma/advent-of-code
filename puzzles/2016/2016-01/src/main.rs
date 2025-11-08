use puzzle_2016_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2016,
        day: 1,
        name: "No Time for a Taxicab",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
