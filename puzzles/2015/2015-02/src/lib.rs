mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    assert_eq!(part_one("2x3x4"), 58);
    assert_eq!(part_one("1x1x10"), 43);

    assert_eq!(part_one(include_str!("../input.txt")), 1588178);
}
