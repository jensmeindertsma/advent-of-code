mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
        \"\"
        \"abc\"
        \"aaa\\\"aaa\"
        \"\\x27\"
    "}),
        12
    );

    assert_eq!(part_one(include_str!("../input.txt")), 1350);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
        \"\"
        \"abc\"
        \"aaa\\\"aaa\"
        \"\\x27\"
    "}),
        19
    );

    assert_eq!(part_two(include_str!("../input.txt")), 2085);
}
