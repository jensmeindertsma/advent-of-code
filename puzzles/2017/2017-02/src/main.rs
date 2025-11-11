use puzzle_2017_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2017,
        day: 2,
        name: "Corruption Checksum",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
