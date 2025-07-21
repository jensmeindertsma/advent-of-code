mod parsing;

mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("turn on 0,0 through 999,999"), 1_000_000);
    assert_eq!(part_one("turn on 0,0 through 999,0"), 1_000);
    assert_eq!(part_one("turn off 499,499 through 500,500"), 0);

    assert_eq!(part_one(include_str!("../input.txt")), 377_891);
}

#[test]
fn two() {
    assert_eq!(part_two("turn on 0,0 through 0,0"), 1);
    assert_eq!(part_two("toggle 0,0 through 999,999"), 2_000_000);

    assert_eq!(part_two(include_str!("../input.txt")), 14_110_788);
}
