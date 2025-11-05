use puzzle_2015_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2015,
        day: 1,
        name: "Not Quite Lisp",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
