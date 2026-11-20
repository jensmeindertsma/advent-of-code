mod part_one;
// mod part_two;

pub use part_one::part_one;
// pub use part_two::part_two;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
            H => HO
            H => OH
            O => HH

            HOH
    "}),
        4
    );

    // assert_eq!(
    //     part_one(indoc! {"
    //         H => HO
    //         H => OH
    //         O => HH

    //         HOHOHO
    // "}),
    //     7
    // );

    // assert_eq!(part_one(include_str!("../input.txt")), 576);
}

// #[test]
// fn two() {
//     assert_eq!(part_two("123"), 3)

//     // assert_eq!(part_two(include_str!("../input.txt")), 9);
// }
