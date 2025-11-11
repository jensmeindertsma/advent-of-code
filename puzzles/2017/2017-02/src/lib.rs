mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
        5 1 9 5
        7 5 3
        2 4 6 8
    "}),
        18
    );

    assert_eq!(part_one(include_str!("../input.txt")), 39126);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
        5 9 2 8
        9 4 7 3
        3 8 6 5
    "}),
        9
    );

    //assert_eq!(part_two(include_str!("../input.txt")), 39126);
}
