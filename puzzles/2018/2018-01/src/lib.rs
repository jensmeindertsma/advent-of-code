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

    assert_eq!(part_one(include_str!("../input.txt")), 445);
}

#[test]
fn two() {
    assert_eq!(part_two("+1, -2, +3, +1"), 2);
    assert_eq!(part_two("+1, -1"), 0);
    assert_eq!(part_two("+3, +3, +4, -2, -4"), 10);
    assert_eq!(part_two("-6, +3, +8, +5, -6"), 5);
    assert_eq!(part_two("+7, +7, -2, -7, -4"), 14);

    assert_eq!(part_two(include_str!("../input.txt")), 219);
}
