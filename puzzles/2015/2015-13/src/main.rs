use puzzle_2015_13::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Knights of the Dinner Table",
        year: 2015,
        day: 13,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
