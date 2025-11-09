use puzzle_2017_01::{part_one, part_two};
use spruce::Puzzle;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    Puzzle {
        year: 2017,
        day: 1,
        name: "Inverse Captcha",
        part_one,
        part_two,
    }
    .solve(INPUT)
}
