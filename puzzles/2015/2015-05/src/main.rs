use puzzle_2015_05::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2015,
        day: 5,
        name: "Doesn't He Have Intern-Elves For This?",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
