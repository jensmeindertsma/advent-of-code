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
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "}),
        2
    );

    assert_eq!(part_one(include_str!("../input.txt")), 396);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "}),
        1
    );

    assert_eq!(part_two(include_str!("../input.txt")), 428);
}
