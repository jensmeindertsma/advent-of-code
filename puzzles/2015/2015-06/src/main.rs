use puzzle_2015_06::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Probably a Fire Hazard",
        year: 2015,
        day: 6,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
