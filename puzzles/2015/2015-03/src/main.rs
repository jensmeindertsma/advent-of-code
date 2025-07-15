use puzzle_2015_03::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2015,
        day: 3,
        name: "Perfectly Spherical Houses in a Vacuum",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
