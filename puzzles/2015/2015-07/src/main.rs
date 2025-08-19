use puzzle_2015_07::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2015,
        day: 7,
        name: "Some Assembly Required",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
