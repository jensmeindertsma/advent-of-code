use puzzle_2021_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2021,
        day: 2,
        name: "Dive!",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
