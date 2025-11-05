mod direction;
mod instruction;

mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("R2, L3"), 5);
    assert_eq!(part_one("R2, R2, R2"), 2);
    assert_eq!(part_one("R5, L5, R5, R3"), 12);

    assert_eq!(part_one(include_str!("../input.txt")), 273);
}

#[test]
fn two() {
    assert_eq!(part_two("R8, R4, R4, R8"), 4);

    assert_eq!(part_two(include_str!("../input.txt")), 115);
}
