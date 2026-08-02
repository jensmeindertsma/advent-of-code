use puzzle_2015_10::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Elves Look, Elves Say",
        year: 2015,
        day: 10,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
