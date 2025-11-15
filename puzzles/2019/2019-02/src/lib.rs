mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    // TODO: test computation input code and output code

    assert_eq!(part_one(include_str!("../input.txt")), 7210630);
}

#[test]
fn two() {
    assert_eq!(part_two("helloworld"), 0);

    //assert_eq!(part_two(include_str!("../input.txt")), 6);
}
