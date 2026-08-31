mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("abcdef"), 609_043);
    assert_eq!(part_one("pqrstuv"), 1_048_970);

    assert_eq!(part_one(include_str!("../input.txt")), 254_575);
}

#[test]
fn two() {
    assert_eq!(part_two(include_str!("../input.txt")), 1_038_736);
}
