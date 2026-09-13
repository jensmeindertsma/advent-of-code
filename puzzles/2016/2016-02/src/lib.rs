mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
            ULL
            RRDDD
            LURDL
            UUUUD
        "}),
        "1985"
    );

    assert_eq!(part_one(include_str!("../input.txt")), "56855");
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
            ULL
            RRDDD
            LURDL
            UUUUD
        "}),
        "5DB3"
    );

    assert_eq!(part_two(include_str!("../input.txt")), "B3C27");
}
