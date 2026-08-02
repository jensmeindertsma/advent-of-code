use puzzle_2015_16::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Aunt Sue",
        year: 2015,
        day: 16,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
