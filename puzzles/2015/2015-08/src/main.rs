use puzzle_2015_08::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Matchsticks",
        year: 2015,
        day: 8,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
