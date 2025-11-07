use puzzle_2015_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2015,
        day: 2,
        name: "I Was Told There Would Be No Math",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
