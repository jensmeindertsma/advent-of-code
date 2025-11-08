use puzzle_2016_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2016,
        day: 2,
        name: "The coolest puzzle template",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
