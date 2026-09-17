use puzzle_2016_03::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Squares With Three Sides",
        year: 2016,
        day: 3,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
