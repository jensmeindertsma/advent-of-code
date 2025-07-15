mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one(">"), 2);
    assert_eq!(part_one("^>v<"), 4);
    assert_eq!(part_one("^v^v^v^v^v"), 2);

    assert_eq!(part_one(include_str!("../input.txt")), 2572);
}

#[test]
fn two() {
    assert_eq!(part_two("^v"), 3);
    assert_eq!(part_two("^>v<"), 3);
    assert_eq!(part_two("^v^v^v^v^v"), 11);

    assert_eq!(part_two(include_str!("../input.txt")), 2631);
}
