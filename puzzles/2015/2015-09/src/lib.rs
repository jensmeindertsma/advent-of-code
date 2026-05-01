mod parsing;
mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

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

    assert_eq!(part_one(include_str!("../input.txt")), 251);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141
    "}),
        982
    );

    assert_eq!(part_two(include_str!("../input.txt")), 898);
}
