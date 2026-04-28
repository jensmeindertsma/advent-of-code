use puzzle_2015_05::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Doesn't He Have Intern-Elves For This?",
        year: 2015,
        day: 5,
    }
    .part_one(part_one)
    .part_two(part_two)
    .solve(INPUT)
}
