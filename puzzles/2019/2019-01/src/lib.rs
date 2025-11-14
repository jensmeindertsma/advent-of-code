mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("12"), 2);
    assert_eq!(part_one("14"), 2);
    assert_eq!(part_one("1969"), 654);
    assert_eq!(part_one("100756"), 33583);

    assert_eq!(part_one(include_str!("../input.txt")), 3474920);
}

#[test]
fn two() {
    assert_eq!(part_two("14"), 2);
    assert_eq!(part_two("1969"), 966);
    assert_eq!(part_two("100756"), 50346);

    assert_eq!(part_two(include_str!("../input.txt")), 5209504);
}
