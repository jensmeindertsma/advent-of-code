use puzzle_2018_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2018,
        day: 1,
        name: "Chronal Calibration",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
