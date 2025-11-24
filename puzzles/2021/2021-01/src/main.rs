use puzzle_2021_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2021,
        day: 1,
        name: "Sonar Sweep",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
