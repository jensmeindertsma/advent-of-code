use puzzle_2015_03::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "Perfectly Spherical Houses in a Vacuum",
        year: 2015,
        day: 3,
    }
    .part_one(part_one)
    .solve(INPUT)
}
