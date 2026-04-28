use puzzle_2015_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Not Quite Lisp",
        year: 2015,
        day: 1,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
