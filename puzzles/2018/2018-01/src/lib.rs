mod parsing;
mod shared;

mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("+1, -2, +3, +1"), 3);
    assert_eq!(part_one("+1, +1, +1"), 3);
    assert_eq!(part_one("+1, +1, -2"), 0);
    assert_eq!(part_one("-1, -2, -3"), -6);

    assert_eq!(part_one(include_str!("../input.txt")), 53);
}

#[test]
fn two() {
    // assert_eq!(part_two("helloworld"), 0);

    // assert_eq!(part_two(include_str!("../input.txt")), 6);
}
