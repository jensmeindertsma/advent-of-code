mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("ugknbfddgicrmopn"), 1);
    assert_eq!(part_one("aaa"), 1);
    assert_eq!(part_one("jchzalrnumimnmhp"), 0);
    assert_eq!(part_one("haegwjzuvuyypxyu"), 0);
    assert_eq!(part_one("dvszwmarrgswjxmb"), 0);

    assert_eq!(part_one(include_str!("../input.txt")), 258);
}

#[test]
fn two() {
    assert_eq!(part_two("qjhvhtzxzqqjkmpb"), 1);
    assert_eq!(part_two("xxyxx"), 1);
    assert_eq!(part_two("uurcxstgmygtbstg"), 0);
    assert_eq!(part_two("ieodomkazucvgmuy"), 0);

    assert_eq!(part_two(include_str!("../input.txt")), 53);
}
