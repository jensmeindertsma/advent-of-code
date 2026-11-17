mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
            aaaaa-bbb-z-y-x-123[abxyz]
            a-b-c-d-e-f-g-h-987[abcde]
            not-a-real-room-404[oarel]
            totally-real-room-200[decoy]
        "}),
        1514
    );

    assert_eq!(part_one(include_str!("../input.txt")), 409147);
}

// #[test]
// fn two() {
//     assert_eq!(part_two("123"), 3)

//     // assert_eq!(part_two(include_str!("../input.txt")), 9);
// }
