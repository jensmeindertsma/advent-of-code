mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141
    "}),
        605
    );

    // assert_eq!(part_one(include_str!("../input.txt")), 251);
}
