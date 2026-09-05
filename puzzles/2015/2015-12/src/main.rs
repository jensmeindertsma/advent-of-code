use puzzle_2015_12::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "JSAbacusFramework.io",
        year: 2015,
        day: 12,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
