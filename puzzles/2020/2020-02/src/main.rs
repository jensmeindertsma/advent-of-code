use puzzle_2020_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2020,
        day: 2,
        name: "Password Philosophy",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
