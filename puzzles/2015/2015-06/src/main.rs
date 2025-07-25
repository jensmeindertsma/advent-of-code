use puzzle_2015_06::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2015,
        day: 6,
        name: "Probably a Fire Hazard",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
