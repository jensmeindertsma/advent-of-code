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
            Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "}),
        62842880
    );

    assert_eq!(part_one(include_str!("../input.txt")), 21367368);
}

#[test]
fn two() {
    use indoc::indoc;

    assert_eq!(
        part_two(indoc! {"
            Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "}),
        57600000
    );

    assert_eq!(part_two(include_str!("../input.txt")), 1766400);
}
