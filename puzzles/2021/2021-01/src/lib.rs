mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "}),
        7
    );

    assert_eq!(part_one(include_str!("../input.txt")), 1226);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "}),
        5
    );

    assert_eq!(part_two(include_str!("../input.txt")), 1252);
}
