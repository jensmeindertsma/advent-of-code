mod generator;
mod part_one;
mod part_two;
mod validate;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("abcdefgh"), "abcdffaa");
    assert_eq!(part_one("ghijklmn"), "ghjaabcc");

    assert_eq!(part_one(include_str!("../input.txt")), "hepxxyzz");
}

#[test]
fn two() {
    assert_eq!(part_two(include_str!("../input.txt")), "heqaabcc");
}
