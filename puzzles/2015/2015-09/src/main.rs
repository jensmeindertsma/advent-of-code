use puzzle_2015_09::part_one;
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        name: "All in a Single Night",
        year: 2015,
        day: 9,
    }
    .part_one(part_one)
    .solve(INPUT)
}
