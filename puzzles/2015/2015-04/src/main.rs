use puzzle_2015_04::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2015,
        day: 4,
        name: "The Ideal Stocking Stuffer",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
