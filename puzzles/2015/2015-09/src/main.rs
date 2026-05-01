use puzzle_2015_09::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "All in a Single Night",
        year: 2015,
        day: 9,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
