use puzzle_2015_18::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Like a GIF For Your Yard",
        year: 2015,
        day: 18,
    }
    .part_one(|input| part_one(input, 100))
    .part_two(|input| part_two(input, 100))
    .solve(INPUT)
}
