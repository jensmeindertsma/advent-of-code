mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
        1721
        979
        366
        299
        675
        1456
    "}),
        514579
    );

    assert_eq!(part_one(include_str!("../input.txt")), 157059);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
        1721
        979
        366
        299
        675
        1456
    "}),
        241861950
    );

    assert_eq!(part_two(include_str!("../input.txt")), 165080960);
}
