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
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "}),
        150
    );

    assert_eq!(part_one(include_str!("../input.txt")), 1580000);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "}),
        900
    );

    // assert_eq!(part_two(include_str!("../input.txt")), ??);
}
