mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("1122"), 3);
    assert_eq!(part_one("1111"), 4);
    assert_eq!(part_one("1234"), 0);
    assert_eq!(part_one("91212129"), 9);

    assert_eq!(part_one(include_str!("../input.txt")), 1034);
}

#[test]
fn two() {
    assert_eq!(part_two("1212"), 6);
    assert_eq!(part_two("1221"), 0);
    assert_eq!(part_two("123425"), 4);
    assert_eq!(part_two("123123"), 12);
    assert_eq!(part_two("12131415"), 4);

    assert_eq!(part_two(include_str!("../input.txt")), 1356);
}
