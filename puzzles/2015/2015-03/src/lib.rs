mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    assert_eq!(part_one(">"), 2);
    assert_eq!(part_one("^>v<"), 4);
    assert_eq!(part_one("^v^v^v^v^v"), 2);

    assert_eq!(part_one(include_str!("../input.txt")), 2572);
}
