mod parsing;
mod part_one;
mod part_two;
mod present;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("2x3x4"), 58);
    assert_eq!(part_one("1x1x10"), 43);

    assert_eq!(part_one(include_str!("../input.txt")), 1588178);
}

#[test]
fn two() {
    assert_eq!(part_two("2x3x4"), 34);
    assert_eq!(part_two("1x1x10"), 14);

    assert_eq!(part_two(include_str!("../input.txt")), 3783758);
}
