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
                Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
            "},
            1000
        ),
        1120
    );

    assert_eq!(part_one(include_str!("../input.txt"), 2503), 2696);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(
            indoc! {"
                Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
            "},
            1000
        ),
        689
    );

    assert_eq!(part_two(include_str!("../input.txt"), 2503), 1084);
}
