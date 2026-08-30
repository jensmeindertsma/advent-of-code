mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    assert_eq!(part_one("123"), 3)

    // assert_eq!(part_one(include_str!("../input.txt")), 9);
}
