mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("helloworld"), 10);

    assert_eq!(part_one(include_str!("../input.txt")), 53);
}

#[test]
fn two() {
    assert_eq!(part_two("helloworld"), 0);

    assert_eq!(part_two(include_str!("../input.txt")), 6);
}
