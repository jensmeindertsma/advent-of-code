use puzzle_2015_19::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Medicine for Rudolph",
        year: 2015,
        day: 19,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
