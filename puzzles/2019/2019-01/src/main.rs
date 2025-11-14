use puzzle_2019_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2019,
        day: 1,
        name: "The Tyranny of the Rocket Equation",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
