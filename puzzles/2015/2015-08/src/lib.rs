mod part_one;

pub use part_one::part_one;

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
