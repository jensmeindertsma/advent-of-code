use puzzle_2016_04::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Security Through Obscurity",
        year: 2016,
        day: 4,
    }
    .part_one(part_one)
    .solve(INPUT)
}
