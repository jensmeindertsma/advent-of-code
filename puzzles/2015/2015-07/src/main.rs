use puzzle_2015_07::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Some Assembly Required",
        year: 2015,
        day: 7,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
