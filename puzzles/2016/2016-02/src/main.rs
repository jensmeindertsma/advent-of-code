use puzzle_2016_02::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Bathroom Security",
        year: 2016,
        day: 2,
    }
    .part_one(part_one)
    .solve(INPUT)
}
