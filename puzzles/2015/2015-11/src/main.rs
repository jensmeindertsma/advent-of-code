use puzzle_2015_11::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Corporate Policy",
        year: 2015,
        day: 11,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
