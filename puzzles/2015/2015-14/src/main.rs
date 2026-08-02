use puzzle_2015_14::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Reindeer Olympics",
        year: 2015,
        day: 14,
    }
    .part_one(|input| part_one(input, 2503))
    .part_two(|input| part_two(input, 2503))
    .solve(INPUT)
}
