mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    assert_eq!(part_one("abcdef"), 609043);
    assert_eq!(part_one("pqrstuv"), 1048970);

    assert_eq!(part_one(include_str!("../input.txt")), 254575);
}
