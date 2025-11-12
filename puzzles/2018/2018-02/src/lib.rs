mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
            abcdef
            bababc
            abbcde
            abcccd
            aabcdd
            abcdee
            ababab
    "}),
        12
    );

    assert_eq!(part_one(include_str!("../input.txt")), 6200);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
            abcde
            fghij
            klmno
            pqrst
            fguij
            axcye
            wvxyz
    "}),
        "fgij"
    );

    assert_eq!(
        part_two(include_str!("../input.txt")),
        "xpysnnkqrbuhefmcajodplyzw"
    );
}
