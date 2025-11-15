use puzzle_2019_02::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2019,
        day: 2,
        name: "1202 Program Alarm",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
