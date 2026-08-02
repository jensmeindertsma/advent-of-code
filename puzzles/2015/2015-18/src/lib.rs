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
                .#.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####..
            "},
            4
        ),
        4
    );

    assert_eq!(part_one(include_str!("../input.txt"), 100), 1061);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(
            indoc! {"
                .#.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####..
            "},
            5
        ),
        17
    );

    assert_eq!(part_two(include_str!("../input.txt"), 100), 1006);
}
