use puzzle_2018_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2018,
        day: 2,
        name: "Inventory Management System",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
