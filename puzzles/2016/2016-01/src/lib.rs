mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    assert_eq!(part_one("R2, L3"), 5);
    assert_eq!(part_one("R2, R2, R2"), 2);
    assert_eq!(part_one("R5, L5, R5"), 12);

    assert_eq!(part_one(include_str!("../input.txt")), 273);
}
