use puzzle_2015_17::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "No Such Thing as Too Much",
        year: 2015,
        day: 17,
    }
    .part_one(|input| part_one(input, 150))
    .part_two(|input| part_two(input, 150))
    .solve(INPUT)
}
