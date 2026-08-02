mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(
            indoc! {"
            20
            15
            10
            5
            5
        "},
            25
        ),
        4
    );

    assert_eq!(part_one(include_str!("../input.txt"), 150), 1304);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(
            indoc! {"
            20
            15
            10
            5
            5
        "},
            25
        ),
        3
    );

    assert_eq!(part_two(include_str!("../input.txt"), 150), 18);
}
