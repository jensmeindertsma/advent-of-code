mod part_one;

pub use part_one::part_one;

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
