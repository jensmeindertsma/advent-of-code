mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    assert_eq!(part_one("turn on 0,0 through 999,999"), 1_000_000);
    assert_eq!(part_one("toggle 0,0 through 999,0"), 1000);
    assert_eq!(part_one("turn off 499,499 through 500,500"), 0);
    assert_eq!(part_one(include_str!("../input.txt")), 377891);
}
