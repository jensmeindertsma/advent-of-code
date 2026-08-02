use puzzle_2015_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "I Was Told There Would Be No Math",
        year: 2015,
        day: 2,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
