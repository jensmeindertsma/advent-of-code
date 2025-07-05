use puzzle_2015_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        part_one,
        part_two: Some(part_two),
    }
    .solve(INPUT);
}
